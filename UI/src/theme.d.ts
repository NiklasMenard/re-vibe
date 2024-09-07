import 'styled-components';

declare module 'styled-components' {
  export interface DefaultTheme {
    devices: {
      xs: string;
      sm: string;
      md: string;
      lg: string;
      xl: string;
    };
    colors: {
      tangelo: string;
      vanilla: string;
      jet: string;
      coral: string;
      viridian: string;
    };
  }
}
