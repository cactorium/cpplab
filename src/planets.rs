pub static PAGE_TEXT: &'static [u8] = b"
<!DOCTYPE html5>
<html>
<style>
textarea {
  border:1px solid #999999;
  font-family:Consolas,Monaco,Lucida Console,Liberation Mono,DejaVu Sans Mono,Bitstream Vera Sans Mono,Courier New, monospace;
}
</style>
<body>
<pre>

#include <cmath>

struct Point {
    double x, y;
};

struct Force {
    double x, y;
};

struct Planet {
    Point position;
    double mass;
};

</pre>
<textarea cols=100 rows=40>
Force CalculateForces(const Planet &a, const Planet &b) {
    /// your code here!
}
</textarea>
</body>
</html>
";

pub fn mod_cpp(s: String) -> String {
    String::from("
#include <iostream>
#include <string>

struct Point {
    double x, y;
};

struct Force {
    double x, y;
};

struct Velocity {
    double x, y;
};

struct Planet {
    Point position;
    double mass;
};

typedef Planet Star;

#ifndef NSTEPS
#define NSTEPS 1000
#endif

#ifndef TIME_STEP
#define TIME_STEP 86400
#endif

Force CalculateForces(const Planet &a, const Planet &b);

int main() {
    Star sun = {1.4960e+11, 0.0, 1.989e+30};
    // geo centric :D
    Planet earth = {0.0, 0.0, 5.972e+24};
    Velocity earthVel = {0.0, 29800};
    for (int = 0; i < NSTEPS; i++) {
        std::cout << earthVel.x << \",\" << earthVel.y << std::endl;
        auto f = CalculateForces(sun, earth);
        earthVel.x += f.x / earth.mass * TIME_STEP;
        earthVel.y += f.y / earth.mass * TIME_STEP;
        earth.position.x += earthVel.x * TIME_STEP;
        earth.position.y += earthVel.y * TIME_STEP;
    }
    return 0;
}
                 ") + s.as_str()
}
