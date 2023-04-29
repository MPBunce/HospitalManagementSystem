Select DISTINCT Affiliated_With.Physician, 
    Affiliated_With.Department, Affiliated_With.PrimaryAffiliation, 
                Department.DepartmentID, Department.Name, Department.Head
FROM Physician
    LEFT JOIN Affiliated_With ON Physician.EmployeeID = Affiliated_With.Physician 
    LEFT JOIN Department ON Affiliated_With.Department = Department.DepartmentID
WHERE
    Physician.EmployeeID = 3





--Query For Docotr Prodecures
Select DISTINCT 
            Trained_In.Physician, Trained_In.Treatment, Trained_In.CertificationDate, Trained_In.CertificationExpires, 
            Procedure.Code, Procedure.Name, Procedure.Cost
FROM Physician
    LEFT JOIN Trained_In ON Physician.EmployeeID = Trained_In.Physician
    LEFT JOIN Procedure ON Trained_In.Treatment = Procedure.Code
WHERE
    Physician.EmployeeID = 3

--Mediucation
SELECT *
FROM Medication

--APP
SELECT *
FROM Appointment
    LEFT JOIN Patient ON Patient.SSN = Appointment.Patient
    LEFT JOIN Prescribes ON Prescribes.Appointment = Appointment.AppointmentID


--Surgery
SELECT *
FROM Undergoes
    LEFT JOIN Patient ON Patient.SSN = Undergoes.Patient
    LEFT JOIN Procedure ON Procedure.Code = Undergoes.Procedure
    LEFT JOIN Stay on Stay.StayID = Undergoes.Stay
    LEFT JOIN Room on Room.Number = Stay.Room
    


SELECT *
FROM Prescribes
    LEFT JOIN Patient on Patient.SSN = Prescribes.Patient