declare module 'vite-plugin-svgr' {
    import { Plugin } from 'vite';
  
    export interface VitePluginSvgrOptions {
      svgo?: boolean;
      svgoConfig?: object;
      include?: string | RegExp | Array<string | RegExp>;
      exclude?: string | RegExp | Array<string | RegExp>;
    }
  
    export default function svgr(
      options?: VitePluginSvgrOptions
    ): Plugin;
  }
  