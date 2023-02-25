<script setup>
import GroupsList from '../components/GroupsList.vue';
import ChatView from './ChatView.vue';
</script>
<template>
    <div id="groups-view">
        <GroupsList></GroupsList>
    </div>
        <ChatView></ChatView>
</template>
<style>
#groups-view {
    border: 1px;
    border-color: bisque;
    width: 30%;
    height: 100%;
}


</style>