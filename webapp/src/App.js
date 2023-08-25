import { Provider } from "react-redux";

import { QueryClient, QueryClientProvider } from "react-query";

import { FluentProvider } from "@fluentui/react-components";

import { store } from "./store";
import { Header, Footer } from "./components";
import { UIRouter } from "./routes/UIRouter";
import { lightBrownTheme } from "./ui/theme";

import "./App.css";

const queryClient = new QueryClient();

function App() {
  return (
    <>
      <FluentProvider theme={lightBrownTheme}>
        <Provider store={store}>
          <QueryClientProvider client={queryClient}>
            <Header />
            <UIRouter />
            <Footer />
          </QueryClientProvider>
        </Provider>
      </FluentProvider>
    </>
  );
}

export default App;
