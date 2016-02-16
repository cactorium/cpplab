extern crate serde;
extern crate serde_json;

pub static PAGE_TEXT: &'static str = "
<!DOCTYPE html5>
<html>
<style>
div {
  border:1px solid #999999;
  font-family:Consolas,Monaco,Lucida Console,Liberation Mono,DejaVu Sans Mono,Bitstream Vera Sans Mono,Courier New, monospace;
}
textarea {
  border:1px solid #999999;
  font-family:Consolas,Monaco,Lucida Console,Liberation Mono,DejaVu Sans Mono,Bitstream Vera Sans Mono,Courier New, monospace;
}
</style>
<script>
var pts = null
var waiting = false
window.onload = function() {
    var button = document.getElementById('submitcode')
    var box = document.getElementById('codebox')
    var cvs = document.getElementById('planets')
    var errorbox = document.getElementById('errorbox')

    var ctx = cvs.getContext('2d')

    button.addEventListener('click', function() {
    if (waiting) {
      return
    }
    var req = new XMLHttpRequest()
    req.open('POST', document.location)
    req.responseType = 'json'
    req.addEventListener('load', function(e) {
        var body = req.response
        if (body.msgs) {
            errorbox.textContent = body.msgs
        } else {
            errorbox.textContent = ''
        }
        if (body.timeout) {
            statusbox.textContent = 'code timed out'
        }
        if (body.failed) {
            statusbox.textContent = 'compilation failed'
        }
        if (body.io_err) {
            statusbox.textContent = 'io error; something really bad happened'
        }
        if (!body.timeout && !body.failed && !body.io_err) {
            statusbox.textContent = 'compiled successfully!'

            // TODO: Draw to canvas
            // figure out the scale
            var points = body.points
            var width = cvs.width, height = cvs.height
            var maxX = Math.max.apply(null, points.map(function(p) { return Math.abs(p.x);}))
            var maxY = Math.max.apply(null, points.map(function(p) { return Math.abs(p.y);}))
            var scale = 0.4*Math.min(width, height)/Math.max(maxX, maxY)

            ctx.clearRect(0, 0, width, height)
            // draw sun
            ctx.fillStyle = 'yellow'
            ctx.strokeStyle = 'yellow'
            ctx.beginPath()
            ctx.arc(width/2, height/2, scale*1.496e+11/8, 0, 2*Math.PI, false)
            ctx.fill()
            // draw orbit
            ctx.strokeStyle = 'black'
            pts = points.map(function(p) {
                return {x: width/2+scale*p.x, y: height/2+scale*p.y}
            })
            ctx.moveTo(pts[0].x, pts[0].y)
            for (var i = 1; i < points.length; i++) {
                ctx.lineTo(pts[i].x, pts[i].y)
            }
            ctx.stroke()
        }
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
<textarea id=codebox cols=100 rows=20>
Force CalculateForces(const Body &a, const Body &b) {
    /// your code here!
}
</textarea>
<p>
<button id=submitcode>Submit!</button>
<p>
<div id=statusbox></div>
<p>
<div id=errorbox style=white-space:pre-line;></div>
<p>
<canvas id=planets width=640 height=640></canvas>
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

#[derive(Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64
}

#[derive(Serialize, Deserialize)]
struct PlanetResults {
    msgs: String,
    points: Vec<Point>
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

pub fn process_out(warnings: String, input: String) -> String {
    let points = input.lines().map(|ln| {
        let split_ln = ln.split(',').collect::<Vec<_>>();
        Point::new(split_ln[0].parse().unwrap(), split_ln[1].parse().unwrap())
    }).collect::<Vec<_>>();

    serde_json::to_string(&PlanetResults{
        msgs: warnings,
        points: points
    }).unwrap()
}
