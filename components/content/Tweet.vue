<template>
    <div id="{{this.divId}}">
      <!-- tweet -->
    </div>
</template>
<script>
export default {
  head: {
        script: [
            { hid: 'twitter', type: 'text/javascript', src: '//platform.twitter.com/widgets.js', async: true, defer: true }
        ]
  },
  props: ['tweetId', 'divId'],
  mounted() {
    if (this.tweetId) {
      twttr.widgets.createTweet(this.tweetId, this.divId)
    } else {
      twttr.widgets.load(this.divId)
    }
  }
}
</script>
