<script setup>
  import { toRefs } from 'vue';

  import KeyCombination from './KeyCombination.vue';
  import SliderRange from './SliderRange.vue';
  import CodeArea from './CodeArea.vue';

  import initiatorTypes from '../data/initiatorTypes.json';

  const props = defineProps(["initiator", "deleteInitiator", "openArgumentsPopup", "index"]);

  const { initiator, deleteInitiator, index } = toRefs(props);

  function setInitiator(initiatorType) {
    initiator.value.type = initiatorType.value;
    initiator.value.data = initiatorType.defaultData;
  }
</script>

<template>
    <div class="initiator">
        <span class="initiatorType">
            <span>{{ initiatorTypes.find(initiatorType => initiatorType.value === initiator.type)?.name || "nothing" }}</span>
            <div class="initiatorSelect">
                <div 
                v-for="initiatorType in initiatorTypes" 
                class="initiatorSelectOption" 
                :class="{selected: initiator.type === initiatorType.value }"
                :key="initiatorType" 
                @click="setInitiator(initiatorType)">
                    <span>{{ initiatorType.name }}</span>
                    <p>{{ initiatorType.description }}</p>
                </div>
            </div>
        </span>
        <template v-if="initiator.type === 'keypress'">
            <span class="initiatorType">
                <span>{{ initiator.data.keys.join(" + ") }}</span>
                <div class="initiatorSelect narrow">
                    <KeyCombination :initiator="initiator"/>
                </div>
            </span>
            <span class="initiatorType">
              <span>{{ initiator.data.activateTime === 'press' ? 'On press' : 'On release' }}</span>
              <div class="initiatorSelect right">
                  <div 
                    v-for="activateTime in [
                      {name: 'On press', value: 'press', description: 'When the key combination is pressed.'},
                      {name: 'On release', value: 'release', description: 'When the key combination is released. Allows you to set a timeframe that triggers the action.'}
                    ]" 
                    class="initiatorSelectOption" 
                    :class="{selected: initiator.type === activateTime.value }"
                    :key="activateTime"
                    @click="initiator.data.activateTime = activateTime.value">
                      <span>{{ activateTime.name }}</span>
                      <p>{{ activateTime.description }}</p>
                  </div>
              </div>
          </span>
          <div v-if="initiator.data.activateTime === 'release'" class="initiatorType slider noArrow">
            <SliderRange :min="0" :max="10" :step="0.1" :defaultValue1="initiator.data.time.min" :defaultValue2="initiator.data.time.max" @change="(e) => {
              initiator.data.time.min = e.value1;
              initiator.data.time.max = e.value2;
            }"/>
          </div>
        </template>
        <!-- <template v-else-if="initiator.type === 'appLaunched'">
            <span class="initiatorType">
                <span>{{ initiator.data.appPath }}</span>
                <div class="initiatorSelect narrow">
                    --><!-- TODO: make this a list of applications instead of a file selected -->
                    <!-- This would require looking through a ton of different directories depending on the OS, though, as far as I can tell
                    <FileSelector :filterName="'executable'" :filterExtensions="['exe', 'app', 'AppImage', 'desktop', '']" @selected="(file) => {
                      if(file.path.endsWith('.desktop')) {
                        readTextFile(file.path).then(content => {
                          let lines = content.split('\n');
                          let appPath = lines.find(line => line.startsWith('Exec='))?.split('=')[1];
                          initiator.data.appPath = appPath;
                        });
                      } else {
                        initiator.data.appPath = file.path;
                      }
                    }" />
                </div>
            </span>
        </template> -->
        <template v-else-if="initiator.type === 'time'">
          <span class="initiatorType noArrow">
            <input type="text" v-model="initiator.data.cron" placeholder="CRON syntax" class="initiatorTypeInput" />
          </span>
        </template>
        <svg 
          class="deleteInitiator" 
          @click="deleteInitiator(initiator)"
          xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
            <path fill="#9b3434" d="M19 4h-3.5l-1-1h-5l-1 1H5v2h14M6 19a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7H6v12Z"/>
        </svg>
        <br />
        <CodeArea :position="{ type: 'Initiator', treePosition: [index] }" :openArgumentsPopup="props.openArgumentsPopup" :executes="initiator.executes" />
    </div>
</template>

<style scoped>
  .deleteInitiator {
    position: absolute;
    right: 16px;
    top: 16px;
    width: 30px;
    height: 30px;
  }
  .deleteInitiator:hover path {
    cursor: pointer;
    fill: #b62d2d;
  }
  .slider {
    position: relative;
    display: inline-block;
    height: 30px;
    --opacity: 0;
  }
  .slider:hover {
    --opacity: 1;
  }
  .initiatorTypeInput {
    position: absolute;
    inset: 0;
    border: none;
    padding: 0;
    font-size: 16px;
    outline: 3px solid transparent;
    background: transparent;
    transition: border-color 0.2s ease-in-out;
    color: var(--primary-text);
  }
  .initiatorTypeInput:hover, .initiatorTypeInput:focus {
    outline: 3px solid var(--secondary-background);
  }
  .initiatorType {
    background-color: var(--primary-background);
    margin: 5px;
    padding: 0 25px 0 10px;
    position: relative;
    width: 250px;
    min-height: 31px;
    display: inline-block;
  }
  .initiatorType:not(.noArrow)::after {
    position: absolute;
    content: "";
    /* Down arrow */
    width: 0;
    height: 0;
    --size: 6px;
    border-left: var(--size) solid transparent;
    border-right: var(--size) solid transparent;
    border-top: var(--size) solid var(--primary-text);
    top: calc(50% - var(--size) / 2);
    right: 5px;
  }
  .initiatorSelect {
    position: absolute;
    background-color: var(--dark-background);
    top: 30px;
    left: -3px;
    width: 620px;
    display: none;
    flex-wrap: wrap;
    z-index: 200;
  }
  .initiatorSelect.right {
    right: -3px;
    left: auto;
  }
  .initiatorSelect.narrow {
    width: 300px;
  }
  .initiatorSelectOption {
    margin: 5px;
    padding: 5px;
    width: 300px;
    background-color: var(--primary-background);
    cursor: pointer;
  }
  .initiatorSelectOption:hover {
    background-color: var(--secondary-background);
  }
  .initiatorSelectOption.selected {
    background-color: var(--secondary-background);
  }
  .initiatorSelectOption span {
    font-size: 40px;
    font-weight: bold;
    color: var(--primary-text);
    margin: 0;
  }
  .initiatorSelectOption p {
    font-size: 20px;
    color: var(--secondary-text);
    margin: 10px;
  }
  .initiatorType:hover .initiatorSelect {
    display: flex;
  }
  .initiator {
    padding: 10px;
    margin: 10px;
    background-color: var(--dark-background);
    position: relative;
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }
  .initiatorType > span, .initiatorSelectOption > span {
    font-size: 20px;
  }
</style>