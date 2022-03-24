import os
import time
from scipy import integrate
import numpy as np
import matplotlib.pyplot as plt

# Read the geometry from a data file
def read_airfoil(name):
    naca_filepath = os.path.join("airfoils", name)
    with open(naca_filepath, 'r') as file:
        x, y = [], []
        for line in file.readlines():
            if line == "\n":
                continue
            line = line.rstrip('\r\n')
            line = line.split(' ')
            read_x = False
            for num in line:
                if num == "":
                    continue
                elif read_x:
                    y.append(float(num))
                else:
                    x.append(float(num))
                    read_x = True

        return np.array(x), np.array(y)

class Panel:
    def __init__(self, xa, ya, xb, yb):
        self.xa, self.ya = xa, ya
        self.xb, self.yb = xb, yb

        self.xc, self.yc = (xa+xb)/2, (ya+yb)/2
        self.length = ((xb - xa)**2 + (yb - ya)**2)**0.5

        #panel orientation
        if xb - xa <= 0.0:
            self.beta = np.arccos((yb - ya) / self.length)
        else:
            self.beta = np.pi + np.arccos(-(yb - ya) / self.length)

        #location of the panel (upper/lower)
        if self.beta <= np.pi:
            self.loc = "upper"
        else:
            self.loc = "lower"

        self.sigma = 0.0 #source strength
        self.vt = 0.0 #tangential velocity
        self.cp = 0.0 #pressure coeff

def define_panels(x, y, N=40):
    R = (x.max() - x.min()) /2
    x_center = (x.max() + x.min()) /2
    x_circle = x_center + R * np.cos(np.linspace(0.0, 2 * np.pi, N+1))

    x_ends = np.copy(x_circle)
    y_ends = np.empty_like(x_ends)

    x, y = np.append(x, x[0]), np.append(y, y[0])

    #y coordinates of end points
    I = 0
    for i in range(N):
        while I < len(x) - 1:
            if(x[I] <= x_ends[i] <= x[I+1]) or (x[I+1] <= x_ends[i] <= x[I]):
                break
            else:
                I+=1
        a = (y[I+1] - y[I]) / (x[I+1] - x[I])
        print(a)
        b = y[I+1] - a * x[I+1]
        y_ends[i] = a*x_ends[i] + b
    y_ends[N] = y_ends[0]

    panels = np.empty(N, dtype=object)
    for i in range(N):
        panels[i] = Panel(x_ends[i], y_ends[i], x_ends[i+1], y_ends[i+1])

    return panels

class Freestream:
    def __init__(self, u_inf=1.0, alpha=0.0):
        self.u_inf = u_inf
        self.alpha = np.radians(alpha)


def integral(x, y, panel, dxdz, dydz):
    def integrand(s):
        return (((x - (panel.xa - np.sin(panel.beta) * s)) * dxdz +
                 (y - (panel.ya + np.cos(panel.beta) * s)) * dydz) /
                ((x - (panel.xa - np.sin(panel.beta) * s))**2 +
                 (y - (panel.ya + np.cos(panel.beta) * s))**2 ))
    return integrate.quad(integrand, 0.0, panel.length)[0]

def source_contribution_normal(panels):
    A = np.empty((panels.size, panels.size), dtype=float)
    # source contribution on a panel from itself
    np.fill_diagonal(A, 0.5)
    # source contribution on a panel from others
    for i, p_i in enumerate(panels):
        for j, p_j in enumerate(panels):
            if i != j:
                A[i, j] = 0.5 / np.pi * integral(p_i.xc, p_i.yc, p_j, np.cos(p_i.beta), np.sin(p_i.beta))

    return A

def vortex_contribution_normal(panels):
    A = np.empty((panels.size, panels.size), dtype=float)
    # source contribution on a panel from itself
    np.fill_diagonal(A, 0.0)
    # source contribution on a panel from others
    for i, p_i in enumerate(panels):
        for j, p_j in enumerate(panels):
            if i != j:
                A[i, j] = -0.5 / np.pi * integral(p_i.xc, p_i.yc, p_j, np.sin(p_i.beta), -np.cos(p_i.beta))

    return A

def kutta_condition(A_source, B_vortex):
    b = np.empty(A_source.shape[0] + 1, dtype=float)
    b[:-1] = B_vortex[0,:] + B_vortex[-1,:]
    b[-1] = -np.sum(A_source[0,:] + A_source[-1,:])

    return b

def build_singularity_matrix(A_source, B_vortex):
    A = np.empty((A_source.shape[0]+1, A_source.shape[1]+1), dtype=float)
    # source contribution matrix
    A[:-1, :-1] = A_source
    # vortex contibution array
    A[:-1, -1] = np.sum(B_vortex, axis=1)
    # Kutta condition
    A[-1, :] = kutta_condition(A_source, B_vortex)

    return A

def build_freestream_rhs(panels, freestream):
    b = np.empty(panels.size+1, dtype=float)

    #freestream contribution
    for i, panel in enumerate(panels):
        b[i] = -freestream.u_inf * np.cos(freestream.alpha - panel.beta)
    # freestream contribution on the Kutta condition
    b[-1] = -freestream.u_inf * (np.sin(freestream.alpha - panels[0].beta) +
                                 np.sin(freestream.alpha - panels[-1].beta))

    return b

def compute_tangential_velocity(panels, freestream, gamma, A_source, B_vortex):
    A = np.empty((panels.size, panels.size+1), dtype=float)
    A[:,:-1] = B_vortex
    A[:, -1] = -np.sum(A_source, axis=1)
    b = freestream.u_inf * np.sin([freestream.alpha - panel.beta for panel in panels])

    strengths = np.append([panel.sigma for panel in panels], gamma)

    tangential_velocities = np.dot(A, strengths) + b

    for i, panel in enumerate(panels):
        panel.vt = tangential_velocities[i]


def get_pressure_coefficients(panels, freestream):
    for panel in panels:
        panel.cp = 1.0 - (panel.vt/freestream.u_inf)**2

def get_velocity_field(panels, freestream, X, Y):
    u = freestream.u_inf * np.cos(freestream.alpha) * np.ones_like(X, dtype=float)
    v = freestream.u_inf * np.sin(freestream.alpha) * np.ones_like(X, dtype=float)

    vec_integral = np.vectorize(integral)
    for panel in panels:
        u += panel.sigma / (2.0 * np.pi) * vec_integral(X, Y, panel, 1.0, 0.0)
        v += panel.sigma / (2.0 * np.pi) * vec_integral(X, Y, panel, 0.0, 1.0)

    return u, v

start_time = time.time()

# Initialize simulation
u_inf = 1.0
alpha = 7.0
freestream = Freestream(u_inf, alpha)

x, y = read_airfoil("naca0012.dat")
N = 100
panels = define_panels(x, y, N)

A_source = source_contribution_normal(panels)
B_vortex = vortex_contribution_normal(panels)
A = build_singularity_matrix(A_source, B_vortex)
b = build_freestream_rhs(panels, freestream)

# solve for singularity strengths
strengths = np.linalg.solve(A, b)

# store source strength on each panel
for i, panel in enumerate(panels):
    panel.sigma = strengths[i]

#store circulation density
gamma = strengths[-1]

compute_tangential_velocity(panels, freestream, gamma, A_source, B_vortex)
get_pressure_coefficients(panels, freestream)

# define mesh grid
nx, ny = 20, 20 #number of points in each direction
x_start, x_end = -1.0, 2.0
y_start, y_end = -0.3, 0.3
X, Y = np.meshgrid(np.linspace(x_start, x_end, nx), np.linspace(y_start, y_end, ny))

u,v = get_velocity_field(panels, freestream, X, Y)

#pressure field
cp = 1.0 - (u**2 + v**2) / freestream.u_inf**2

#testing the accuracy
accuracy = sum([panel.sigma * panel.length for panel in panels])
print(f"sum of source/sink strengths: {accuracy}")

#compute Lift coef
chord = abs(max(panel.xa for panel in panels) -
            min(panel.xa for panel in panels))
cl = (gamma * sum(panel.length for panel in panels) / (0.5 * freestream.u_inf * chord))
print(f"lift coefficient: CL = {cl}")

print(f"--- Took {time.time() - start_time} seconds ---")


plt.figure()
#Pressure field
contf = plt.contourf(X,Y, cp,
                       levels=np.linspace(-2.0, 1.0, 100), extend="both")
cbar = plt.colorbar(contf, orientation="vertical",
                    shrink=0.5, pad=0.1,
                    ticks=[-1.0, -1.0, 0.0, 1.0])
cbar.set_label("$C_p$", fontsize=16)
# Panels
#plt.plot( np.append([panel.xa for panel in panels], panels[0].xa), np.append([panel.ya for panel in panels], panels[0].ya), linestyle='-', linewidth=1, marker='o', markersize=3, color='r')
# Velocity field
plt.streamplot(X,Y,u,v, density=0.7, linewidth=0.5, arrowsize=0.5, arrowstyle='->', color="k")
# Profile
plt.fill(x,y, color='k', linestyle='solid', linewidth=2, zorder=2)
plt.xlim(x_start, x_end)
plt.ylim(y_start, y_end)
plt.xlabel("x", fontsize=16)
plt.ylabel("y", fontsize=16)
plt.axis("scaled")
plt.title(f"Streamlines and Pressure field around a NACA 0012 airfoil (AoA=${alpha}^\circ$)", fontsize=16)
plt.show()
plt.savefig("sourcepanels_naca0012.jpg", dpi=300)

# Cp graph
plt.figure(figsize=(10,6))
plt.grid()
plt.xlabel('x', fontsize=16)
plt.ylabel('$C_p$', fontsize=16)
plt.plot([panel.xc for panel in panels if panel.loc == 'upper'],
            [panel.cp for panel in panels if panel.loc == 'upper'],
            label='upper',
            color='r', linewidth=1, markersize=8)
plt.plot([panel.xc for panel in panels if panel.loc == 'lower'],
            [panel.cp for panel in panels if panel.loc == 'lower'],
            label='lower surface',
            color='b', linewidth=1, markersize=6)
plt.legend(loc='best', prop={"size":14})
plt.xlim(-0.1, 1.1)
plt.ylim(1.0, 2.0)
plt.title(f"Number of panels: {N}")
#plt.show()
