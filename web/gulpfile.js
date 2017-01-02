var gulp = require('gulp');
var elm  = require('gulp-elm');
var exec = require('gulp-exec');
var mkdirp = require('mkdirp');

gulp.task('elm-init', elm.init);

gulp.task('compile-elm', ['elm-init'], function() {
  gulp.src('src/Main.elm').pipe(elm()).pipe(gulp.dest('build/'));
});

gulp.task('compile-elm-css', ['elm-init'], function() {
  var options = {
    outputDir: "build/css"
  };

  mkdirp(options.outputDir, function(err) {
    if(err) {
      console.log("Failed to create directory [" + options.outputDir + "]");
      process.exit(1);
    }
  });

  var reportOptions = {};
  return gulp.src('stylesheet/Stylesheets.elm')
    .pipe(exec('elm-css <%= file.path %> --output <%= options.outputDir %>', options))
    .pipe(exec.reporter(reportOptions));
});