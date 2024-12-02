import { type Component, Show } from "solid-js";

// deno-lint-ignore ban-types
const NotFoundPage: Component<{}> = (
  _props,
) => {
  return (
    <Show when={window !== undefined}>
      <div>
        <h1>404 - Not Found</h1>
        <p>Sorry, the page you are looking for does not exist.</p>
      </div>
    </Show>
  );
};

export default NotFoundPage;
