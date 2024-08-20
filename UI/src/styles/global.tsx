import { createGlobalStyle, ThemeProvider, DefaultTheme } from 'styled-components';
import { PropsWithChildren } from 'react';

const GlobalStyle = createGlobalStyle`

body {
    margin: 0; 
    padding: 0; 
    border: 0;
    background-color: #333333;
}

html {
    height: 100svh;
    scroll-behavior:smooth
}

#root {
    font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;
  color-scheme: light dark;
  color: rgba(255, 255, 255, 0.87);
  background-color: #242424;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  height: 100vh;
}

ul {
    padding: 0;
    list-style-type: none;
    margin: 0;
}


*, *::after, *::before {
    box-sizing: border-box;
  }

  
input, select, div {
    -webkit-box-sizing: border-box;
       -moz-box-sizing: border-box;
            box-sizing: border-box;
            appearance: none;
            -webkit-appearance: none;
            -moz-appearance: none;
}

h1 { font-size: 2rem; }
h2 { font-size: 1.5rem; }
h3 { font-size: 1.17rem; }
h4 { font-size: 1.00rem; }
h5 { font-size: 0.83rem; }
h6 { font-size: 0.67rem; }

img, video, canvas {
  height: auto;
}

h1, h2, h3, h4, p, a, span, li {
  color: #eeeeee;
  letter-spacing: 0.1em;
}

h1, h2, h3 {
  line-height: 1.3;
}

span, p {
  line-height: 1.75;

}

img { border-style: none; }

h1 {
  font-size: clamp(1.2rem,  -0.875rem + 8.333vw, 2.1rem);
}

h2, h3, a, li {
  font-size: clamp(1.2rem, -0.875rem + 8.333vw, 1.5rem);
}

p {
  font-size: clamp(1rem, -0.875rem + 1rem, 1.5rem);
}

a{
  display: block;
  text-decoration: none;
  position: relative;
  outline: none;
  -webkit-tap-highlight-color: transparent;
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

a::after {
    content: '';
    background: white;
    mix-blend-mode: exclusion;
    width: calc(100% + 1rem);
    height: 0;
    position: absolute;
    bottom: -0.25rem;
    left: -0.5rem;
    border-radius: 0.75rem;
    transition: all 0.3s cubic-bezier(0.445, 0.05, 0.55, 0.95);
  }

  @media (min-width: 1024px) {
    a:hover::after {
    height: calc(100% + 0.5rem);
  }
  }

`;

const breakpoints = {
  xs: '320px',
  sm: '640px',
  md: '768px',
  lg: '1024px',
  xl: '1280px',
  '2xl': '1536px',
};

const theme: DefaultTheme = {
  devices: {
    xs: `(min-width: ${breakpoints.xs})`,
    sm: `(min-width: ${breakpoints.sm})`,
    md: `(min-width: ${breakpoints.md})`,
    lg: `(min-width: ${breakpoints.lg})`,
    xl: `(min-width: ${breakpoints.xl})`,
  },
  colors: {
    moonGrey: 'rgb(51, 51, 51)',
    shamrock: 'rgb(73, 209, 169)',
    melrose: 'rgb(114, 103, 225)',
    YlnmnBlue: 'rgb(73, 81, 111)',
    mediumPurple: 'rgb(114, 103, 225)',
  },
};

const Theme = ({ children }: PropsWithChildren<Record<string, unknown>>) => (
  <ThemeProvider theme={theme}>{children}</ThemeProvider>
);

export { GlobalStyle, Theme };
