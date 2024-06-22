<script>
export default {
  data() {
    return {
      options: ['', ''],
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
      this.result = this.options[randomIndex] // TODO: only choose from the options that are non-empty
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
  <div class="px-4 py-5 my-5 text-center">
  <h1 class="display-5 fw-bold text-body-emphasis"> Pick For Me</h1>
  <p class="lead mb-4">Instant Decisions</p>
    <div class="row">
    <div class="col-md-6 mx-auto col-lg-5">
        <div v-for="(textBox, index) in options" :key="index" >
          <div class="form-floating mb-3">
            <input type="text" class="form-control" :id="'textBox' + index" v-model="options[index]">
            <label :for="'textBox' + index">Option {{ index + 1 }}</label>
          </div>
        </div>
      <button class="btn btn-secondary" @click="addOptionBox">+ Add Option</button>
      <button class="btn btn-primary" @click="pickRandom"> Pick For Me</button>
    </div>
      <div class="container">
        <div class="row mt-3">
          <h3>Picked for you: {{ result }}</h3>
        </div>
        <div class="row mt-3">
          <h4>{{ this.info }}</h4>
        </div>
      </div>
    </div>
  </div>
  
</template>
