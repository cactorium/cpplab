pub static PAGE_TEXT: &'static str = "
<!DOCTYPE html5>
<html>
<style>
textarea {
  border:1px solid #999999;
  font-family:Consolas,Monaco,Lucida Console,Liberation Mono,DejaVu Sans Mono,Bitstream Vera Sans Mono,Courier New, monospace;
}
</style>
<script>
var waiting = false
window.onload = function() {
  document.getElementById('submitcode').addEventListener('click', function() {
    if (waiting) {
      return
    }
    var req = new XMLHttpRequest()
    req.open('POST', document.location)
    req.addEventListener('load', function(e) {
    })
    req.addEventListener('error', function(e) {
    })
    req.addEventListener('loadend', function() {
      console.log('request completed')
      waiting = false
    })
    req.send(document.getElementById('codebox').value)
  })
}
</script>

<body>
<pre>

#include &lt;cmath&gt;

struct Point {
  double x, y;
};

struct Vector {
  double x, y;
};

typedef Vector Force;

struct Body {
  Point position;
  double mass;
};

</pre>
<textarea id=codebox cols=100 rows=40>
Force CalculateForces(const Body &a, const Body &b) {
    /// your code here!
}
</textarea>
<p>
<button id=submitcode>Submit!</button>
<p>
<div id=errorbox></div>
<p>
<canvas id=planets></canvas>
</body>
</html>
";

pub fn mod_cpp(s: String) -> String {
    String::from("
#include <cmath>
#include <iostream>
#include <string>

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
  auto sun = Body{Point{0.0, 0.0}, 1.989e+30};
  // geo centric :D
  auto earth = Body{Point{1.4960e+11, 0.0}, 5.972e+24};
  auto earthVel = Velocity{0.0, 29800.0};
  // earthVel.x = 0.0;
  // earthVel.y = 29800;
  for (int i = 0; i < NSTEPS; i++) {
    auto f = CalculateForces(earth, sun);
    std::cout << 
      earth.position.x << \",\" << earth.position.y << std::endl;

    earthVel.x += f.x / earth.mass * TIME_STEP;
    earthVel.y += f.y / earth.mass * TIME_STEP;
    earth.position.x += earthVel.x * TIME_STEP;
    earth.position.y += earthVel.y * TIME_STEP;
  }
  return 0;
}

                 ") + s.as_str()
}


pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

pub fn process_out(warnings: String, input: String) -> String {
    String::from("foo!")
}
