{% if install_command is string -%}
```bash
{{ install_command }}
```
{%- elif install_command is iterable -%}
{%- for cmd in install_command %}
```bash
{{ cmd }}
```
{%- endfor %}
{%- endif %}
