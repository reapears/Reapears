import { Provider } from "react-redux";

import { QueryClient, QueryClientProvider } from "react-query";

import { FluentProvider, webLightTheme } from "@fluentui/react-components";

import { store } from "./store";
import { Header, Footer } from "./components";
import { UIRouter } from "./routes/UIRouter";
import "./App.css";
import { lightBrownTheme } from "./ui/theme";

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
