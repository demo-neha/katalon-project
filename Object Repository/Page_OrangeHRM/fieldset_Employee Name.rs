<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>fieldset_Employee Name</name>
   <tag></tag>
   <elementGuidId>7764e254-6cf6-48ff-bbdf-66ed0b80a5fa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>fieldset</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

                
                    Employee Name
  





        

            var employees_empsearch_employee_name = [];

            $(document).ready(function() {
            
                var nameField = $(&quot;#empsearch_employee_name_empName&quot;);
                var idStoreField = $(&quot;#empsearch_employee_name_empId&quot;);
                var typeHint = 'Type for hints...';
                var hintClass = 'inputFormatHint';
                var loadingMethod = 'ajax';
                var loadingHint = 'Loading';
            
                if (idStoreField.val() != '') {
                    idStoreField.data('item.name', nameField.val());
                }
                
                nameField.data('typeHint', typeHint);
                nameField.data('loadingHint', loadingHint);
                
                nameField.one('focus', function() {

                        if ($(this).hasClass(hintClass)) {
                            $(this).val(&quot;&quot;);
                            $(this).removeClass(hintClass);
                        }

                    });
                    
                if( loadingMethod != 'ajax'){
                    if (nameField.val() == '' || nameField.val() == typeHint) {
                        nameField.val(typeHint).addClass(hintClass);
                    }

                    

                    nameField.autocomplete(employees_empsearch_employee_name, {

                        formatItem: function(item) {
                            return $('&lt;div/>').text(item.name).html();
                        },
                        formatResult: function(item) {
                            return item.name
                        }
                      ,matchContains:true
                        }).result(function(event, item) {
                            idStoreField.val(item.id);
                            idStoreField.data('item.name', item.name);
                        }

                    );
                 }else{
                        var value = nameField.val().trim();
                        nameField.val(loadingHint).addClass('ac_loading');
                        $.ajax({
                               url: &quot;/index.php/pim/getEmployeeListAjax&quot;,
                               data: '',
                               dataType: 'json',
                               success: function(employeeList){

                                     nameField.autocomplete(employeeList, {

                                                formatItem: function(item) {
                                                    return $('&lt;div/>').text(item.name).html();
                                                },
                                                formatResult: function(item) {
                                                    return item.name
                                                }
                                                
                                                ,matchContains:true
                                            }).result(function(event, item) {
                                                idStoreField.val(item.id);
                                                idStoreField.data('item.name', item.name);
                                            }

                                        );
                                         nameField.removeClass('ac_loading'); 
                                        
                                         if(value==''){
                                            nameField.val(typeHint).addClass(hintClass);
                                         } else {
                                            nameField.val(value).addClass();
                                         }
                                    }
                             });
                 }
                
            }); // End of $(document).ready

                 
        



Id
  

Employment Status
  
All
Freelance
Full-Time Contract
Full-Time Permanent
Full-Time Probation
Part-Time Contract
Part-Time Internship


Include
  
Current Employees Only
Current and Past Employees
Past Employees Only


Supervisor Name
  

Job Title
  
All
Account Clerk
CEO
Finance Manager
HR Executive
HR Manager
IT Executive
IT Manager
Sales Executive
Sales Manager


Sub Unit
  
All
Sales
Administration
IT
Finance



                

                
                                 

                
                    
                                        
                

            </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;search_form&quot;)/fieldset[1]</value>
   </webElementProperties>
</WebElementEntity>
