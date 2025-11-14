/* @refresh reload */
import { render } from 'solid-js/web'
import App from './App.tsx'
import './ssr-values'

const root = document.getElementById('root')

render(() => <App />, root!)
