<script setup>
    import * as store from '@/store';
    import { useRouter } from 'vue-router';

    let router = useRouter();

    store.get("userWelcomed", false).then(value => {
        if(value) {
            router.push("/macros");
        } else {
            store.set("userWelcomed", true);
            router.push("/welcome");
        }
    });
</script>

<template>
    <div>
        Loading...
    </div>
</template>