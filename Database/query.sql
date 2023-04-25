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


--Undergoes
SELECT * 
FROM Patient
    LEFT JOIN Undergoes ON Undergoes.Patient = Patient.SSN
    LEFT JOIN Stay ON Stay.StayID = Undergoes.Stay
    LEFT JOIN Room ON Room.Number = stay.room
WHERE Undergoes.Patient IS NOT NULL

--Does not Undergoes
SELECT * 
FROM Patient
    LEFT JOIN Undergoes ON Undergoes.Patient = Patient.SSN
    LEFT JOIN Stay ON Stay.StayID = Undergoes.Stay
WHERE Undergoes.Patient IS NULL

SELECT *
FROM Undergoes

--Mediucation
SELECT *
FROM Medication

DELETE
FROM Undergoes
WHERE Physician = 3