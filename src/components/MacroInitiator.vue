<script setup>
  import { toRefs } from 'vue';
  import KeyCombination from './KeyCombination.vue';
  import SliderRange from './SliderRange.vue';
  const props = defineProps(["initiator", "deleteInitiator"]);

  const { initiator, deleteInitiator } = toRefs(props);

  const initiatorTypes = [
    {name: "Keypress", description: "Do something when a certain key combination is used.", value: "keypress", defaultData: { keys: ["a"], time: { min: 20, max: 80 } }},
    {name: "Application Launched", description: "Do something when a certain application is launched.", value: "appLaunched", defaultData: { appPath: "/" }},
  ];

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
        <div v-if="initiator.type === 'keypress'" class="initiatorData">
            <span class="initiatorType">
                <span>{{ initiator.data.keys.join(" + ") }}</span>
                <div class="initiatorSelect narrow">
                    <KeyCombination :initiator="initiator"/>
                </div>
            </span>
            <!-- <SliderRange :min="0" :max="10" :step="0.1" :defaultValue1="initiator.data.time.min" :defaultValue2="initiator.data.time.max" /> -->
        </div>
        <svg 
        class="deleteInitiator" 
        @click="deleteInitiator(initiator)"
        xmlns="http://www.w3.org/2000/svg" aria-hidden="true" role="img" width="1em" height="1em" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
            <path fill="#9b3434" d="M19 4h-3.5l-1-1h-5l-1 1H5v2h14M6 19a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7H6v12Z"/>
        </svg>
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
  .initiatorData {
    display: inline-block;
  }
  .initiatorType {
    border: 3px solid #141a2766;
    margin: 5px;
    padding: 0 25px 0 10px;
    position: relative;
    width: 250px;
    display: inline-block;
  }
  .initiatorType::after {
    position: absolute;
    content: "";
    /* Down arrow */
    width: 0;
    height: 0;
    --size: 6px;
    border-left: var(--size) solid transparent;
    border-right: var(--size) solid transparent;
    border-top: var(--size) solid #fff;
    top: calc(50% - var(--size) / 2);
    right: 5px;
  }
  .initiatorSelect {
    position: absolute;
    background-color: #222a3a99;
    top: 28px;
    left: -3px;
    width: 620px;
    display: none;
    flex-wrap: wrap;
  }
  .initiatorSelect.narrow {
    width: 300px;
  }
  .initiatorSelectOption {
    margin: 5px;
    padding: 5px;
    width: 300px;
    background-color: #191b1f99;
    cursor: pointer;
  }
  .initiatorSelectOption:hover {
    background-color: #191b1fcc;
  }
  .initiatorSelectOption.selected {
    background-color: #1f2229cc;
  }
  .initiatorSelectOption span {
    font-size: 40px;
    font-weight: bold;
    color: #fff;
    margin: 0;
  }
  .initiatorSelectOption p {
    font-size: 20px;
    color: #ddd;
    margin: 10px;
  }
  .initiatorType:hover .initiatorSelect {
    display: flex;
  }
  .initiator {
    padding: 10px;
    margin: 10px;
    border: 3px solid #141a27;
    position: relative;
  }
  .initiatorType > span, .initiatorSelectOption > span {
    font-size: 20px;
  }
</style>