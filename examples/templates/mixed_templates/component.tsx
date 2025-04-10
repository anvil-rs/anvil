import React from 'react';

interface {{ name }}Props {
{% for prop in props %}
  {{ prop }};
{% endfor %}
}

export const {{ name }} = (props: {{ name }}Props) => {
  return (
    <div>
      <h2>{{ name }}</h2>
      <ul>
      {% for prop in props %}
        <li>{{ prop }}</li>
      {% endfor %}
      </ul>
    </div>
  );
}

export default {{ name }};
