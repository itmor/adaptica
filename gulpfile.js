const gulp = require("gulp");
const less = require("gulp-less");
const cleanCSS = require("gulp-clean-css");
const rename = require("gulp-rename");
const path = require("path");

// Используйте require вместо import
const axios = require('axios');

// Переменная, чтобы отслеживать состояние BrowserSync
let isBrowserSyncRunning = false;

const reloadBrowserSync = () => {
  axios.post("http://localhost:3004/__browser_sync__?method=reload")
    .then(() => {
      isBrowserSyncRunning = true;
      console.log('Reload page');
    })
    .catch(() => isBrowserSyncRunning = false);
};

gulp.task("compile-less", () => {
  return gulp
    .src(["src/modules/**/*.less", "src/shared/**/*.less"], { base: "src" })
    .pipe(less())
    .pipe(cleanCSS({ compatibility: "ie8" }))
    .pipe(
      rename((path) => {
        path.basename = path.basename + ".css";
        path.extname = "";
      })
    )
    .pipe(gulp.dest("src"))
    .on("end", reloadBrowserSync);
});

gulp.task("watch", () => {
  gulp.watch(["src/modules/**/*.less", "src/shared/**/*.less"], gulp.series("compile-less"));
});

gulp.task("default", gulp.series("compile-less", "watch"));
