<template>
    <div>
        <h1>Doctor</h1>
        <div>
            <div>{{doctor}}</div>
            <ul>{{ doctor.employee_id }}</ul>
            <ul>{{ doctor.name }}</ul>
            <ul>{{ doctor.position }}</ul>
            <ul>{{ doctor.ssn }}</ul>
        </div>
        
        <h1>Departments</h1>
        <div>
            <div>{{departments}}</div>
            <ul>{{ departments.employee_id }}</ul>
            <ul>{{ departments.department }}</ul>
            <ul>{{ departments.primary_affiliation }}</ul>
            <ul>{{ departments.department_id }}</ul>
            <ul>{{ departments.name }}</ul>
            <ul>{{ departments.head }}</ul>

        </div>

        <h1>Medical Procedures</h1>
        <div>
            <div>{{ procedures }}</div>
            <ul>{{ procedures.employee_id }}</ul>
            <ul>{{ procedures.treatment }}</ul>
            <ul>{{ procedures.certification_date }}</ul>
            <ul>{{ procedures.certification_exp }}</ul>
            <ul>{{ procedures.code }}</ul>
            <ul>{{ procedures.name }}</ul>
            <ul>{{ procedures.cost }}</ul>

        </div>

    </div>
</template>

<script setup>
    import axios from 'axios';


    const { employee_id } = useRoute().params
    let input_employee_id = employee_id


    let doctor = await axios.get(`http://localhost:1080/api/get_physician/${input_employee_id}`)
    doctor = doctor.data

    console.log(doctor)

    const departments = ref([]);

    onMounted( async () => {
        try{
            const department_response = await axios.get(`http://localhost:1080/api/get_departments/${input_employee_id}`);
            departments.value = department_response.data;
            console.log(departments.value)
        }catch (error) {
            procedures.value = ["No associated departments"];
            console.log(error);
        }
    });



    const procedures = ref([]);

    onMounted(async () => {
        try {
            const response = await axios.get(`http://localhost:1080/api/get_procedures/${input_employee_id}`);
            procedures.value = response.data;
            console.log(procedures.value)
        } catch (error) {
            procedures.value = ["Doctor cannot preform any treatments"];
            console.log(error);
        }
    });


</script>
