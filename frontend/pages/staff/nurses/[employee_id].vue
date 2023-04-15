<template>
    <div>
        {{ employee_id }}
    </div>
</template>

<script setup>
    const { employee_id } = useRoute().params
</script>
