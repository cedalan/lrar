<script setup>
    import TenantView from '@/views/TenantView.vue';
    import BurnTableComponent from './components/BurnTableComponent.vue';
    import NoteBoardComponent from './components/NoteBoardComponent.vue';
    import HeaderComponent from './components/HeaderComponent.vue';
    import {ref} from 'vue';
    const reload_burn_component_variable = ref(0);

    function trigger_burn_component_reload() {
      if (reload_burn_component_variable.value === 1){
        reload_burn_component_variable.value--
      }else {
        reload_burn_component_variable.value++
      }
    }

</script>
<template>
    <div id="everything">
        <HeaderComponent />
        <div id="top-row">
            <div id="tenant-view-container">
                <TenantView @burn-submitted="trigger_burn_component_reload"/>
            </div>
            <div id="board-container">
                <div id="burn-table-container">
                    <BurnTableComponent :key="reload_burn_component_variable"/>
                </div>
                <div id="note-board-container">
                    <NoteBoardComponent />
                </div>
            </div>
        </div>
        <div id="bottom-row">
            <p>Made by Lars Jaatun, Jonas Mørkland and Christian Elias Anderssen Dalan</p>
        </div>
    </div>
</template>

<style>
h1 {
    text-align: center;
}
#everything {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    box-sizing: border-box;
    padding: 10px;
}

#top-row {
    display: flex;
    flex-direction: row;
}

#tenant-view-container {
    display: flex;
    justify-content: center;
    align-items: center;
    flex: 3;
}

#board-container {
    display: flex;
    flex-direction: column;
    flex: 1;
}

#burn-table-container {
    display: flex;
    width: 100%;
    height: 100%;
    flex: 1;
}
#note-board-container {
    flex: 1;
}

#bottom-row {
    display: flex;
    justify-content: center;
    align-items: center;
}

body {
  transform: scale(0.85);
  transform-origin: top left;
  width: 115%;
}

html {
    overflow: hidden;
}
</style>
