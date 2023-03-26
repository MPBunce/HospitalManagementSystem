--Script to generate hospital db for SQLite database

CREATE DATABASE hospital; 

USE hospital

--Create Doctor Table
CREATE TABLE Doctor(
    doctor_id int PRIMARY KEY,
    name text,
    position text,
    ssn
)

--Create Department Table
CREATE TABLE Department(
    department_id int PRIMARY KEY,
    name text,
    head text,
)

CREATE TABLE Affiliated_With(
    doctor_id int,
    department_id int,
    primary_affiliation int,
)

CREATE TABLE Procedure(
    code int,
    name text, 
    cost float,
)

CREATE TABLE Trained_IN(
    doctor_id int,
    treatment text,
    certification_date text,
    certification_expires text,
)

CREATE TABLE Patient(
    patient_id int PRIMARY KEY,
    name text,
    address text,
    phone int,
    insurance_id int,
    pcp int, 
)

CREATE TABLE Nurse(
    nurse_id int PRIMARY KEY,
    name text,
    position text,
    registered bool,
    ssn int,
    
)

CREATE TABLE Appointment(

)

CREATE TABLE Medication(

)

CREATE TABLE Prescribes(

)

CREATE TABLE Block(

)

CREATE TABLE Room(

)

CREATE TABLE On_Call(

)

CREATE TABLE Stay(

)

CREATE TABLE Undergoes(

)