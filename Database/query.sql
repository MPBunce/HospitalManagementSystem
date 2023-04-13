Select *
FROM Physician
    LEFT JOIN Trained_In ON Physician.EmployeeID = Trained_In.Physician
    LEFT JOIN Procedure ON Trained_In.Treatment = Procedure.Code
    LEFT JOIN Affiliated_With ON Physician.EmployeeID = Affiliated_With.Physician 
    LEFT JOIN Department ON Affiliated_With.Department = Department.DepartmentID


SELECT *
FROM Trained_In