Select DISTINCT --Physician.EmployeeID, Physician.Name, Physician.Position, Physician.SSN, 
                --Trained_In.Physician, Trained_In.Treatment, Trained_In.CertificationDate, Trained_In.CertificationExpires, 
                --Procedure.Code, Procedure.Name, Procedure.Cost, 
                Affiliated_With.Physician, Affiliated_With.Department, Affiliated_With.PrimaryAffiliation, 
                Department.DepartmentID, Department.Name, Department.Head
FROM Physician
    --LEFT JOIN Trained_In ON Physician.EmployeeID = Trained_In.Physician
    --LEFT JOIN Procedure ON Trained_In.Treatment = Procedure.Code
    LEFT JOIN Affiliated_With ON Physician.EmployeeID = Affiliated_With.Physician 
    LEFT JOIN Department ON Affiliated_With.Department = Department.DepartmentID
    --LEFT JOIN Physician ON Physician.EmployeeID = Department.Head
WHERE
    Physician.EmployeeID = 3
--LEFT JOIN Physician ON Physician.EmployeeID = Department.Head







--Query For Docotr Prodecures
Select DISTINCT 
            Trained_In.Physician, Trained_In.Treatment, Trained_In.CertificationDate, Trained_In.CertificationExpires, 
            Procedure.Code, Procedure.Name, Procedure.Cost
FROM Physician
    LEFT JOIN Trained_In ON Physician.EmployeeID = Trained_In.Physician
    LEFT JOIN Procedure ON Trained_In.Treatment = Procedure.Code
WHERE
    Physician.EmployeeID = 3