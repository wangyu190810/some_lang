var gulp = require("gulp");
var babel = require("gulp-babel");
 
gulp.task("default", function () {
  return gulp.src("src/*.js")
    .pipe(babel())
    .pipe(gulp.dest("dist"));
});
gulp.task('auto', function () {
    // 监听文件修改，当文件被修改则执行 script 任务
    gulp.watch('src/js/**/*.js', ['toes5']);
    gulp.watch('dist/*.js', ['min']);

});