<script setup>
    import TenantView from '@/views/TenantView.vue';
    import BurnTableComponent from './components/BurnTableComponent.vue';
    import NoteBoardComponent from './components/NoteBoardComponent.vue'
</script>
<template>
    <div id="everything">
        <h1 id="welcome-text">Welcome to LRAR - Lazy Roomate Avoidance Registry!</h1>
        <div id="top-row">
            <div id="tenant-view-container">
                <TenantView />
            </div>
            <div id="board-container">
                <div id="burn-table-container">
                    <BurnTableComponent />
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
</style>
