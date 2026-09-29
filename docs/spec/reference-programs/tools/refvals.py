"""Reference values for the first-slice reference programs.

Analytic values in Decimal (50 digits). The drag projectile uses classical RK4
in Decimal with Richardson extrapolation between step sizes; landing located by
secant iteration on the final step length. Float DOPRI5 and RK4 runs estimate
energy drift for pendulum and spring-mass (provisional bounds only; the DOPRI5
step controller here is a simple one and differs in detail from any runtime's).

Run: python refvals.py   (standard library only)
"""
from decimal import Decimal as D, getcontext
import math

getcontext().prec = 50
PI = D("3.14159265358979323846264338327950288419716939937510")


def dsin(x):
    x = x % (2 * PI)
    s, term, n = D(0), x, 1
    while abs(term) > D(10) ** -55:
        s += term
        term = -term * x * x / ((n + 1) * (n + 2))
        n += 2
    return s


def dcos(x):
    return dsin(x + PI / 2)


g = D("9.81")

# ---- RP-01 projectile, no drag -------------------------------------------
speed, ang = D(20), PI / 4
vx, vy = speed * dcos(ang), speed * dsin(ang)
T = 2 * vy / g
print("RP-01 v0x", vx)
print("RP-01 T  ", T)
print("RP-01 R  ", vx * T)
print("RP-01 H  ", vy * vy / (2 * g))
print("RP-01 t_apex", vy / g)
ang60 = PI / 3
vx6, vy6 = speed * dcos(ang60), speed * dsin(ang60)
print("RP-01 60deg T", 2 * vy6 / g, "R", vx6 * 2 * vy6 / g)

# ---- RP-02 projectile, quadratic drag ------------------------------------
k = D("0.01")


def f(s):
    x, y, u, v = s
    sp = (u * u + v * v).sqrt()
    return (u, v, -k * sp * u, -g - k * sp * v)


def rk4(s, h):
    a = f(s)
    b = f(tuple(si + h / 2 * ai for si, ai in zip(s, a)))
    c = f(tuple(si + h / 2 * bi for si, bi in zip(s, b)))
    d = f(tuple(si + h * ci for si, ci in zip(s, c)))
    return tuple(si + h / 6 * (ai + 2 * bi + 2 * ci + di) for si, ai, bi, ci, di in zip(s, a, b, c, d))


def drag_run(h):
    s, t = (D(0), D(0), vx, vy), D(0)
    apex = None
    while True:
        n = rk4(s, h)
        if apex is None and n[3] < 0:
            # locate apex (vy = 0) by secant on step length
            lo, hi = D(0), h
            flo, fhi = s[3], n[3]
            for _ in range(60):
                m = hi - fhi * (hi - lo) / (fhi - flo)
                fm = rk4(s, m)[3]
                lo, flo, hi, fhi = hi, fhi, m, fm
                if abs(fm) < D(10) ** -40:
                    break
            apex = (t + hi, rk4(s, hi))
        if n[1] < 0 and t > 0:
            lo, hi = D(0), h
            flo, fhi = s[1], n[1]
            for _ in range(60):
                m = hi - fhi * (hi - lo) / (fhi - flo)
                fm = rk4(s, m)[1]
                lo, flo, hi, fhi = hi, fhi, m, fm
                if abs(fm) < D(10) ** -40:
                    break
            land = rk4(s, hi)
            return t + hi, land, apex
        s, t = n, t + h


res = {}
for h in (D("0.001"), D("0.0005")):
    res[h] = drag_run(h)
(t1, s1, a1), (t2, s2, a2) = res[D("0.001")], res[D("0.0005")]
# Richardson for order 4: x* = x2 + (x2 - x1)/15
rich = lambda x1, x2: x2 + (x2 - x1) / 15
print("RP-02 T  ", rich(t1, t2), "diff", t2 - t1)
print("RP-02 R  ", rich(s1[0], s2[0]), "diff", s2[0] - s1[0])
print("RP-02 vx_land", rich(s1[2], s2[2]))
print("RP-02 vy_land", rich(s1[3], s2[3]))
print("RP-02 t_apex", rich(a1[0], a2[0]))
print("RP-02 H  ", rich(a1[1][1], a2[1][1]))

# ---- RP-03 bouncing ball --------------------------------------------------
h0, e = D(1), D("0.8")
t1b = (2 * h0 / g).sqrt()
v1 = (2 * g * h0).sqrt()
print("RP-03 t1", t1b, "v1", v1)
tb = t1b
times = [tb]
for kk in range(1, 12):
    tb = tb + 2 * (e ** kk) * v1 / g
    times.append(tb)
for i, tt in enumerate(times[:10], 1):
    print(f"RP-03 bounce {i} t={tt}")
tinf = t1b * (1 + e) / (1 - e)
print("RP-03 t_inf", tinf)
# first occurrence index n whose interval to previous < 1e-6 s
n = 1
while True:
    n += 1
    interval = 2 * (e ** (n - 1)) * v1 / g
    if interval < D("1e-6"):
        break
tn = t1b + 2 * v1 / g * e * (1 - e ** (n - 1)) / (1 - e)
print("RP-03 settle at occurrence", n, "t", tn, "interval", interval, "t_inf - t", tinf - tn)

# ---- RP-04 pendulum -------------------------------------------------------
L = 1.0
gf = 9.81
th0 = math.radians(10)
kk = math.sin(th0 / 2)
a, b = 1.0, math.sqrt(1 - kk * kk)
for _ in range(40):
    a, b = (a + b) / 2, math.sqrt(a * b)
K = math.pi / (2 * a)
Texact = 4 * math.sqrt(L / gf) * K
T0 = 2 * math.pi * math.sqrt(L / gf)
print("RP-04 T0 %.15f  T %.15f  ratio-1 %.6e" % (T0, Texact, Texact / T0 - 1))


def pend(s):
    th, w = s
    return [w, -(gf / L) * math.sin(th)]


def spring(s):
    x, v = s
    return [v, -4.0 * x]


def rk4f(fun, s, h):
    a = fun(s)
    b = fun([si + h / 2 * ai for si, ai in zip(s, a)])
    c = fun([si + h / 2 * bi for si, bi in zip(s, b)])
    d = fun([si + h * ci for si, ci in zip(s, c)])
    return [si + h / 6 * (ai + 2 * bi + 2 * ci + di) for si, ai, bi, ci, di in zip(s, a, b, c, d)]


C = [0, 1 / 5, 3 / 10, 4 / 5, 8 / 9, 1, 1]
A = [[], [1 / 5], [3 / 40, 9 / 40], [44 / 45, -56 / 15, 32 / 9],
     [19372 / 6561, -25360 / 2187, 64448 / 6561, -212 / 729],
     [9017 / 3168, -355 / 33, 46732 / 5247, 49 / 176, -5103 / 18656],
     [35 / 384, 0, 500 / 1113, 125 / 192, -2187 / 6784, 11 / 84]]
B5 = [35 / 384, 0, 500 / 1113, 125 / 192, -2187 / 6784, 11 / 84, 0]
B4 = [5179 / 57600, 0, 7571 / 16695, 393 / 640, -92097 / 339200, 187 / 2100, 1 / 40]


def dopri(fun, s, tend, rtol=1e-6, atol=1e-9):
    t, h = 0.0, 1e-3
    out = [(t, s)]
    while t < tend:
        h = min(h, tend - t)
        ks = []
        for i in range(7):
            si = [s[j] + h * sum(A[i][m] * ks[m][j] for m in range(i)) for j in range(len(s))]
            ks.append(fun(si))
        y5 = [s[j] + h * sum(B5[m] * ks[m][j] for m in range(7)) for j in range(len(s))]
        y4 = [s[j] + h * sum(B4[m] * ks[m][j] for m in range(7)) for j in range(len(s))]
        err = math.sqrt(sum(((y5[j] - y4[j]) / (atol + rtol * max(abs(s[j]), abs(y5[j])))) ** 2 for j in range(len(s))) / len(s))
        if err <= 1:
            t, s = t + h, y5
            out.append((t, s))
        h = h * min(5, max(0.2, 0.9 * (1 / max(err, 1e-10)) ** 0.2))
    return out


def energy_p(s):
    th, w = s
    return 0.5 * (L * w) ** 2 + gf * L * (1 - math.cos(th))  # per unit mass


def energy_s(s):
    x, v = s
    return 0.5 * v * v + 0.5 * 4.0 * x * x


for name, fun, s0, en in (("RP-04 pendulum", pend, [th0, 0.0], energy_p), ("RP-05 spring", spring, [0.1, 0.0], energy_s)):
    E0 = en(s0)
    s, drift = s0, 0.0
    for i in range(10000):
        s = rk4f(fun, s, 0.01)
        drift = max(drift, abs(en(s) - E0) / E0)
    print(name, "rk4 h=0.01 over 100 s: max rel energy drift %.3e" % drift)
    out = dopri(fun, s0, 100.0)
    drift = max(abs(en(st) - E0) / E0 for _, st in out)
    print(name, "dopri5 1e-6/1e-9 over 100 s: max rel energy drift %.3e, steps %d" % (drift, len(out) - 1))
print("RP-05 exact x(10 s) %.15f  T %.15f  E0 %.3f J" % (0.1 * math.cos(20.0), math.pi, 0.02))
print("RP-07 |sum| %.16f" % math.sqrt(13))
