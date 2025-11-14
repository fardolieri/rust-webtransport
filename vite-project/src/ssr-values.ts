declare global {
  interface Window {
    SSR_VALUES: {
      HASH: Uint8Array;
      WEBTRANSPORT_PORT: number;
    }
  }
}

window.SSR_VALUES = {} as any;
window.SSR_VALUES.HASH = null as unknown as Uint8Array;
window.SSR_VALUES.WEBTRANSPORT_PORT = null as unknown as number;
