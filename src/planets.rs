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
    String::from(" ")
}
