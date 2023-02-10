<script setup>
import GroupsList from '../components/GroupsList.vue';
import ChatView from './ChatView.vue';
</script>
<template>
    <div id="groups-view">
        <GroupsList></GroupsList>
    </div>
    <div id="chat-view">
        <ChatView></ChatView>
    </div>
</template>
<style>
#groups-view {
    border: 1px;
    border-color: bisque;
    width: 40%;
    height: 100%;
}

#chat-view {
    width: 60%;
    height: 100%;
    background-color: azure;
    display: flex;
    flex-direction: row;
}
</style>