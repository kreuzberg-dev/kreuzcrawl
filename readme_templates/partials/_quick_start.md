{% if snippets and snippets.basic_usage %}
{{ snippets.basic_usage | include_snippet(language) }}
{% else %}
See the [API Reference](https://docs.kreuzcrawl.dev) for detailed usage.
{% endif %}
