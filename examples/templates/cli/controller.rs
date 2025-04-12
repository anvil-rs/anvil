// CLI Controller Template
// This file is auto-generated

pub struct {{ name }}Controller {
    name: String,
}

impl {{ name }}Controller {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn run(&self) {
        println!("Running {} controller", self.name);
        
        // Generated controller logic for {{ name }}
        {% for action in actions %}
        self.{{ action }}();
        {% endfor %}
    }

    {% for action in actions %}
    fn {{ action }}(&self) {
        println!("  - Executing action: {{ action }}");
    }
    {% endfor %}
}
