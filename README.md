# swivel


## Ethos

- Configuration in code.
- User level abstraction should be minimal. It is the responsibility of the framework to handle the complexity.
- Each component should be interchangeable with __no__ code change.
- The library provides the building blocks, not the solutions.
- Rely on other's implementations.

## Notes

A pluggable web framework

Matching CLI called revolve?

Maybe worth exploring this model: https://github.com/tokio-rs/axum/discussions/2184


It may also be worth looking into supporting different architectures: https://github.com/microsoft/cookiecutter-rust-actix-clean-architecture/blob/main/README.md

If support a variety of architectures, create some type of templating mechanism to generate boilerplate.

Separation of concerns:

- Model
  - Interacts with the database
  - Therefore it should only be interacting with the database, and returning data.
- Views
  - Interacts with the user
  - Therefore it should only be interacting with the user, and returning data.
  - The templating engine manages this. interchangeable templating engines.
- Controllers
  - Interacts with the model and the view
  - Therefore it should only be interacting with the model and view, and returning data.
  - Controller should be the only part that interacts with the backend.


In the MVC architecture at least, each of the three components are indepdendant of each other, except for the interop between.

Models and views should not be aware of each other (except maybe types), and controllers should be the glue between the two.


MVC approaches are pretty universal and can be converted later on into DDD approaches and the like. 


Perfectly translatable concepts:
- Actix scopes to Axum nested routes.
- Axum state and actix data.


Things that we will need to take into consideration:
- Handling different types of url parsing.
  - actix uses `/{user}/test` whereas axum uses `/:user/test`.
- Extractors for said URLS 
- Response types
- Middleware
- Request types
- custom FromRequest that can be implemented for our types.
- Implement FromRequest for ALL MODELS!
- Allows us to pass in some stuff.


- Create own FromRequest trait
- Implement FromRequest on extractors.
- Create Request struct.
- Create mapping from Request to backend Request.
- Create mapping From backend Request to Request.
- For any T that implements our own FromRequest, we can then implement the backend FromRequest for it.
- Alternatively, for every extractor, we implement our local FromRequest.



Approach to extractors:
- In Actix, the FromRequest trait has access to a reference of the entire httpRequest. Given that they have a custom implementation of the HttpRequest, they can then extract the data from the request.
- In axum, they rely on the http crate for their type definitions. To get the request as a http crate type. This type is the http::Parts type. This type is essentially the same (under the hood its just a big struct referencing all componenets).

If we reimplement all the common extractors on our own types, for whatever backend we have, it should be a lot nicer.

To make life significantly easier, I think that it would jist be easier to use the http crate, as that defines a really nice Request and Response values. This would allow us to use the http crate for our own request and response types, and then we can just convert these to the appropriate types for the backend.

the http create also provides us with methods.


```mermaid
flowchart TB
    subgraph Routes

        subgraph Paths
            Extract_Params_Agnostically
        end

        Paths --> Http_Method
        Http_Method --> Handlers
    end
    subgraph Backend

        Handler_Converter --> Register_Routes
        
        Initialize_Http_Server --> Register_Routes
        Register_Routes --> Run_Server

        Path_Converter --> Http_Method_Converter
        Http_Method_Converter --> Handler_Converter
    end

    subgraph View
        Template --> Render
    end

    subgraph Model
        Fetch_Data --> Format_Data
    end

    subgraph Controller
        Controller_Handlers
        Register_Local_Routes
    end

    Controller_Handlers --> Fetch_Data
    Format_Data --> Controller_Handlers

    Controller_Handlers --> Template
    Controller_Handlers --> Register_Local_Routes

    Render --> Controller_Handlers

    Register_Local_Routes --> Routes

    Handlers --> Handler_Converter
    Http_Method --> Http_Method_Converter
    Paths --> Path_Converter

```

What should the DX be?
- Create a backend in the main file.
  - Backend provides:
    - Ability to start the server.
    - Ability to add routes.
    - If we have a route type, then we can just map from our route types to the route types of the backend that we choose.



Backend trait should define API for variety of things:
  - Adding routes
  - Starting the server
  - Adding middleware
  - Adding state
  - Setting Headers
  - Setting Cookies




### User facing APIs that we need to implement:

- Router
- Handlers
- Extractors
- Request / response types.
- Middleware
- State (not really just a parser for handing it down to backend)

We only really need to deal with the user facing APIs. Implementing stuff like handlers is overkill for our purposes and would be a waste of time realistically.

How should things be structured?

In my opinion, a service should take in an input with a request type, and return a future with a
response type.

The service itself should be responsible for calling it's own handlers etc.
A handler should merely take in the arguments that it requires, and run it's asynchronous
operations on those arguments.

This means that the service should be responsible for converting the request into the arguments
that it's handlers require.

This is similar to what actix does under the hood, and I think that it is better practice and an
overall nicer abstraction.
