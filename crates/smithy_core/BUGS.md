* <h1> encompasses the match
```rs
<h1 />
    {
      match *(*app_state.unwrapped_posts).borrow() {
        PromiseState::Pending => smd!(<span>still loading</span>),
        PromiseState::Success(ref post) => {
          smd!(<span>fetched a post with title <b>{ &post.title }</b></span>)
        },
        PromiseState::Error(_) => smd!(<span>Something went wrong fetching the data</span>),
      }
    } // </div>
    <div />
```