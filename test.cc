#include <cmath>
#include <iostream>
#include <string>

#include <stdlib.h>

#include <sys/prctl.h>     /* prctl */
#include <linux/seccomp.h> /* seccomp's constants */
#include <unistd.h>        /* for syscall */
#include <sys/syscall.h>   /* For SYS_xxx definitions */


struct Point {
  double x, y;
};

struct Vector {
  double x, y;
};

// struct Force: Vector {};
// struct Velocity: Vector {};
typedef Vector Force;
typedef Vector Velocity;

struct Body {
  Point position;
  double mass;
};

#ifndef NSTEPS
#define NSTEPS 1000
#endif

#ifndef TIME_STEP
#define TIME_STEP 86400
#endif

Force CalculateForces(const Body &a, const Body &b);

int main() {
  std::cout << " ";
  prctl(PR_SET_SECCOMP, SECCOMP_MODE_STRICT);
  auto sun = Body{Point{0.0, 0.0}, 1.989e+30};
  // geo centric :D
  auto earth = Body{Point{1.4960e+11, 0.0}, 5.972e+24};
  auto earthVel = Velocity{0.0, 29800.0};
  // earthVel.x = 0.0;
  // earthVel.y = 29800;
  for (int i = 0; i < NSTEPS; i++) {
    auto f = CalculateForces(earth, sun);
    std::cout << 
      // f.x << "," << f.y << ";\t" << 
      earth.position.x << "," << earth.position.y << std::endl;

    earthVel.x += f.x / earth.mass * TIME_STEP;
    earthVel.y += f.y / earth.mass * TIME_STEP;
    earth.position.x += earthVel.x * TIME_STEP;
    earth.position.y += earthVel.y * TIME_STEP;
  }

  std::cout.flush();
  syscall(SYS_exit, 0);
}

Force CalculateForces(const Body &a, const Body &b) {
  auto dx = b.position.x - a.position.x;
  auto dy = b.position.y - a.position.y;
  auto dist = std::sqrt(dx*dx + dy*dy);
  const auto G = 6.67408e-11;
  auto mag = G*a.mass*b.mass/(dist*dist);
  auto norm = Vector{dx/dist, dy/dist};
  return Force {mag*norm.x, mag*norm.y};
}
