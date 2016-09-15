/**
 * You are expected to load dust, register the templates, and set the following variables:
 *   - templateName
 *   - context
 */
dust.render(templateName, context, function(err, out) {
  if (err) {
    console.error(err);
  }
  if (out) {
    console.log(out);
  }
});
