<script setup>
    import store from '@/store';
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
      <div v-for="macro in macros" class="macro" :key="macro" @click="selectMacro(macro)">
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
    border: 3px solid var(--accent);
    margin: 10px;
    cursor: pointer;
  }
  .macro:hover {
    border: 3px solid var(--accent-light);
  }
  .macroDescription {
    font-size: 12px;
    color: var(--secondary-text);
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
    color: var(--secondary-text);
  }
  .addMacroButton:hover {
    color: var(--primary-text);
  }
</style>