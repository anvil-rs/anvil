

# Inspiration:

- https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust/
 - "axum becomes an implementation detail, concealed by our own HTTP package."
 - "Business logic is encapsulated by the Service trait and injected into the handler."



Things that might need some impl detail:
 - Making controllers more generic so that we can pass in anything that we want?
 - making background jobs more generic so we can switch out the backend without much fuss.





Some nice-to-haves:
 - 
