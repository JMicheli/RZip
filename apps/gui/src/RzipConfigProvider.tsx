import React from "react";
import { ConfigProvider, theme } from "antd";

interface RzipConfigProviderProps {
  children?: React.ReactNode;
}

/**
 * An implementation of the Ant Design `ConfigProvider` with custom theming
 * for the RZip GUI app. Intended to wrap the `RzipApp` component.
 */
function RzipConfigProvider({ children }: RzipConfigProviderProps) {
  return (
    <ConfigProvider
      theme={{
        algorithm: theme.darkAlgorithm,
        components: {
          Button: {
            colorPrimary: "#228B22"
          }
        }
      }}
    >
      {children}
    </ConfigProvider>
  );
}

export default RzipConfigProvider;
