<script setup>
    import * as store from '@/store';
    import { ref } from 'vue';

    import { useRouter } from 'vue-router';

    let router = useRouter();
    
    let macrosLoaded = ref(false);
    let macros = ref([]);

    store.get('macros', []).then(data => {
      macros.value = data;
      macrosLoaded.value = true;
    });

    function selectMacro(macro) {
      let index = macros.value.indexOf(macro);
      macro.index = index;

      router.push(`/macro/${index}`)
    }

    function addMacro() {
        const currentMacros = macros.value;
        const newMacro = {
            name: 'New Macro',
            description: 'New description',
            macro: {
                initiators: [],
                functions: []
            }
        };
        currentMacros.push(newMacro);
        store.set('macros', currentMacros);
    }
</script>

<template>
    <h1>Macros</h1>
    <div class="noMacros" v-if="macrosLoaded && macros.length === 0">
      <p>No macros found. Add a new one?</p>
      <button class="addMacro button" @click="addMacro">Add macro</button>
    </div>
    <div v-else-if="macrosLoaded">
      <div v-for="macro in macros" class="macro" :class="{ activeMacro: macro === selectedMacro }" :key="macro" @click="selectMacro(macro)">
        <div class="macroName">{{ macro.name }}</div>
        <div class="macroDescription">{{ macro.description }}</div>
      </div>
      <div class="addMacroButton macro" @click="addMacro">
        Add macro
      </div>
    </div>
    <div class="loading" v-else>
      <div class="loadingText">
        Loading macros...
      </div>
    </div>
</template>

<style scoped>
  .macro {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 10px;
    border: 3px solid #242935;
    margin: 10px;
    cursor: pointer;
  }
  .macro:hover {
    border: 3px solid #292f3d;
  }
  .activeMacro {
    border: 3px solid #40495f;
  }
  .activeMacro:hover {
    border: 3px solid #40495f;
  }
  .macroDescription {
    font-size: 12px;
    color: #8e959c;
  }
  h1 {
    text-align: center;
    margin: 0;
  }
  .noMacros {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding-bottom: 100px;
  }
  .addMacro {
    --defaultBG: #2c9094;
    --defaultBoxBG: #237477;
    --hoverBG: #2fa3a3;
    --hoverBoxBG: #2a8a8d;
    --activeBG: #23787e;
    --activeBoxBG: #1c5f5f;
  }
  .addMacroButton {
    height: 50px;
    font-size: 30px;
    font-weight: 500;
    color: #bbb;
  }
  .addMacroButton:hover {
    color: #fff;
  }
</style>