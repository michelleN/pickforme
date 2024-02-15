<script>
export default {
  data() {
    return {
      options: [''],
      result: '',
      info: '',
    }
  },
  watch: {
    result() {
      this.fetchInfo()
    }
  },
  methods: {
    addOptionBox() {
      this.options.push('')
    },
    pickRandom() {
      const randomIndex = Math.floor(Math.random() * this.options.length)
      this.result = this.options[randomIndex]
    },
    async fetchInfo(){
      try {
        const response = await fetch('/result/info', {
          method: 'POST',
          headers: {
            'Content-Type': 'text/plain'
          },
          body: this.result // TODO: sanitize
        });
        if (response.ok) {
          console.log('response ok')
          const textData = await response.text()
          this.info = textData
        } else {
          console.log('response not ok')
        }
      } catch (error) {
        console.error(error); // TODO: deal with error
      }
    }
  }
}
</script>

<template>
  <h1> Pick For Me</h1>

  <h2>Options:</h2>
  <div v-for="(option, index) in options" :key="index">
    <div v-if="index > 1">OR</div>
    <input type="text" @keyup.enter="addOptionBox" placeholder="Add Option" v-model="options[index]">
    <div v-if="index === 0">OR</div>
  </div>
  <button @click="addOptionBox">+</button>
  <div>
  <button @click="pickRandom"> Pick For Me</button>
  <p>This result is: {{ result }}</p>
  <p>{{ this.info }}</p>
  </div>
  
</template>
