/* @refresh reload */
import { generateHydrationScript, renderToString } from 'solid-js/web';

import App from './App';

export function render() {
  return {
    body: renderToString(() => <App />),
    head: generateHydrationScript({})
  }
}
