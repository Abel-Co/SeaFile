<template>
  <n-loading-bar-provider>
    <n-dialog-provider>
      <DialogContent/>
      <n-notification-provider>
        <n-message-provider>
          <MessageContent/>
          <slot name="default"></slot>
        </n-message-provider>
      </n-notification-provider>
    </n-dialog-provider>
  </n-loading-bar-provider>
</template>

<script setup>
import {NDialogProvider, NLoadingBarProvider, NNotificationProvider, NMessageProvider} from 'naive-ui'
import DialogContent from "./DialogContent.vue"
import MessageContent from "./MessageContent.vue"
</script>