import { Provider } from "react-redux";

import { QueryClient, QueryClientProvider } from "react-query";

import { FluentProvider } from "@fluentui/react-components";

import { store } from "./store";

import { UIRouter } from "./routes/UIRouter";
import { lightBrownTheme } from "./themes";

const queryClient = new QueryClient();

function App() {
  return (
    <>
      <FluentProvider theme={lightBrownTheme}>
        <Provider store={store}>
          <QueryClientProvider client={queryClient}>
            <UIRouter />
          </QueryClientProvider>
        </Provider>
      </FluentProvider>
    </>
  );
}

export default App;
