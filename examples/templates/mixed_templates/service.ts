/**
 * {{ name }} - TypeScript Service
 * 
 * Auto-generated service class
 */

export class {{ name }} {
  {% for method in methods %}
  async {{ method }}() {
    // Implementation for {{ method }}
    return await fetch(`/api/${this.resourceName}`);
  }
  {% endfor %}

  private get resourceName() {
    return '{{ name | downcase }}';
  }
}
