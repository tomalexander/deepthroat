var compiled = dust.compile(template, 'compiled');
dust.loadSource(compiled);
dust.render('compiled', context, function(err, out) {
  if (err) {
    console.error(err);
  }
  if (out) {
    console.log(out);
  }
});
