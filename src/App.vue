<!--<script setup lang="ts">-->
<!--import HelloWorld from './components/HelloWorld.vue'-->
<!--</script>-->

<!--<template>-->
<!--  <div>-->
<!--    <a href="https://vitejs.dev" target="_blank">-->
<!--      <img src="/vite.svg" class="logo" alt="Vite logo" />-->
<!--    </a>-->
<!--    <a href="https://vuejs.org/" target="_blank">-->
<!--      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />-->
<!--    </a>-->
<!--    <a href="https://v1.tauri.app/" target="_blank">-->
<!--      <img src="/home/chema/RustroverProjects/rust_calculator/src-tauri/icons/icon.png" class="logo" alt="Tauri logo" />-->
<!--    </a>-->
<!--  </div>-->
<!--  <HelloWorld msg="Vite + Vue + Tauri" />-->
<!--</template>-->

<!--<style scoped>-->
<!--.logo {-->
<!--  height: 6em;-->
<!--  padding: 1.5em;-->
<!--  will-change: filter;-->
<!--  transition: filter 300ms;-->
<!--}-->
<!--.logo:hover {-->
<!--  filter: drop-shadow(0 0 2em #646cffaa);-->
<!--}-->
<!--.logo.vue:hover {-->
<!--  filter: drop-shadow(0 0 2em #42b883aa);-->
<!--}-->
<!--</style>-->

<template>
  <div id="app">
    <h1>Rust Desktop Calculator</h1>
    <div class="calculator">
      <input type="text" v-model="input" readonly/>
      <div class="buttons">
        <button @click="append('7')">7</button>
        <button @click="append('8')">8</button>
        <button @click="append('9')">9</button>
        <button @click="append('/')">/</button>
        <button @click="append('4')">4</button>
        <button @click="append('5')">5</button>
        <button @click="append('6')">6</button>
        <button @click="append('*')">*</button>
        <button @click="append('1')">1</button>
        <button @click="append('2')">2</button>
        <button @click="append('3')">3</button>
        <button @click="append('-')">-</button>
        <button @click="append('0')">0</button>
        <button @click="append('.')">.</button>
        <button @click="calculate()">=</button>
        <button @click="clear()">C</button>
        <button @click="append('+')">+</button>
      </div>
    </div>
  </div>
</template>

<script>
import {invoke} from '@tauri-apps/api';

export default {
  name: 'App',
  data() {
    return {
      input: '',
    }
  },
  methods: {
    append(value) {
      this.input += value;
    },
    async calculate() {
      try {
        this.input = await invoke('calculate', { expression: this.input });
        console.log(this.input);
      } catch (error) {
        this.input = error.toString();
      }
    },
    clear() {
      this.input = '';
    }
  }
}
</script>

<style>
#app {
  text-align: center;
  margin-top: 50px;
}
.calculator {
  display: inline-block;
  border: 1px solid #ccc;
  padding: 20px;
  border-radius: 5px;
}
input {
  width: 100%;
  height: 40px;
  font-size: 24px;
  text-align: right;
  margin-bottom: 10px;
}
.buttons {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
}
button {
  height: 40px;
  font-size: 18px;
}
.error {
  color: red;
  margin-top: 10px;
}
</style>


