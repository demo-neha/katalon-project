<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Employee Information</name>
   <tag></tag>
   <elementGuidId>b0c7cfc6-099a-4b4e-940a-c298eacf99c7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>content</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

                  

    
        Employee Information
    
    
        

            

                
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



                

                
                                 

                
                    
                                        
                

            

        

     

    >

 


    
        
    
    

                
            
    
                 

        


         
        

        
            
            
                
        
        
            
                                
                Id
First (&amp; Middle) Name
Last Name
Job Title
Employment Status
Sub Unit
Supervisor
            
                            

            
                                            
                                                    667
                                                    Raj
                                                    Its Now Or Never
                                                    Account Clerk
                                                    Part-Time Internship
                                                    Finance
                                                    Steven  Edwards
                                            
                                
                            
         
                  
                
        
     
        
 




                    

                        var rootPath = '/';

                        $(document).ready(function() {
                            ohrmList_init();

                        });
                                              





  
    ×
    OrangeHRM - Confirmation Required
  
  
    Delete records?
  
  
    
    
  




    $(document).ready(function() {
        
        var supervisors = [{&quot;name&quot;:&quot;Linda Anderson&quot;,&quot;id&quot;:&quot;1&quot;},{&quot;name&quot;:&quot;Robert Craig&quot;,&quot;id&quot;:&quot;6&quot;},{&quot;name&quot;:&quot;Steven Edwards&quot;,&quot;id&quot;:&quot;4&quot;},{&quot;name&quot;:&quot;Thomas Fleming&quot;,&quot;id&quot;:&quot;5&quot;},{&quot;name&quot;:&quot;John Smith&quot;,&quot;id&quot;:&quot;3&quot;}];

        $('#btnDelete').attr('disabled', 'disabled');

        $(&quot;#ohrmList_chkSelectAll&quot;).click(function() {
            if ($(&quot;:checkbox&quot;).length == 1) {
                $('#btnDelete').attr('disabled', 'disabled');
            }
            else {
                if ($(&quot;#ohrmList_chkSelectAll&quot;).is(':checked')) {
                    $('#btnDelete').removeAttr('disabled');
                } else {
                    $('#btnDelete').attr('disabled', 'disabled');
                }
            }
        });

        $(':checkbox[name*=&quot;chkSelectRow[]&quot;]').click(function() {
            if ($(':checkbox[name*=&quot;chkSelectRow[]&quot;]').is(':checked')) {
                $('#btnDelete').removeAttr('disabled');
            } else {
                $('#btnDelete').attr('disabled', 'disabled');
            }
        });

        // Handle hints
        if ($(&quot;#empsearch_id&quot;).val() == '') {
            $(&quot;#empsearch_id&quot;).val('Type Employee Id...')
                    .addClass(&quot;inputFormatHint&quot;);
        }

        if ($(&quot;#empsearch_supervisor_name&quot;).val() == '') {
            $(&quot;#empsearch_supervisor_name&quot;).val('Type for hints...')
                    .addClass(&quot;inputFormatHint&quot;);
        }

        $(&quot;#empsearch_id, #empsearch_supervisor_name&quot;).one('focus', function() {

            if ($(this).hasClass(&quot;inputFormatHint&quot;)) {
                $(this).val(&quot;&quot;);
                $(this).removeClass(&quot;inputFormatHint&quot;);
            }
        });

        $(&quot;#empsearch_supervisor_name&quot;).autocomplete(supervisors, {
            formatItem: function(item) {
                return $('&lt;div/>').text(item.name).html();
            },
            formatResult: function(item) {
                return item.name
            }
            , matchContains: true
        }).result(function(event, item) {
        }
        );

        $('#searchBtn').click(function() {
            $(&quot;#empsearch_isSubmitted&quot;).val('yes');
            $('#search_form input.inputFormatHint').val('');
            $('#search_form input.ac_loading').val('');
            $('#search_form').submit();
        });

        $('#resetBtn').click(function() {
            $(&quot;#empsearch_isSubmitted&quot;).val('yes');
            $(&quot;#empsearch_employee_name_empName&quot;).val('');
            $(&quot;#empsearch_supervisor_name&quot;).val('');
            $(&quot;#empsearch_id&quot;).val('');
            $(&quot;#empsearch_job_title&quot;).val('0');
            $(&quot;#empsearch_employee_status&quot;).val('0');
            $(&quot;#empsearch_sub_unit&quot;).val('0');
            $(&quot;#empsearch_termination&quot;).val('1');
            $('#search_form').submit();
        });

        $('#btnAdd').click(function() {
            location.href = &quot;/index.php/pim/addEmployee&quot;;
        });
        $('#btnDelete').click(function() {
            $('#frmList_ohrmListComponent').submit(function() {
                $('#deleteConfirmation').dialog('open');
                return false;
            });
        });

        /* Delete confirmation controls: Begin */
        $('#dialogDeleteBtn').click(function() {
            document.frmList_ohrmListComponent.submit();
        });
        /* Delete confirmation controls: End */

    }); //ready

    function submitPage(pageNo) {
        document.frmEmployeeSearch.pageNo.value = pageNo;
        document.frmEmployeeSearch.hdnAction.value = 'paging';
        $('#search_form input.inputFormatHint').val('');
        $('#search_form input.ac_loading').val('');
        $(&quot;#empsearch_isSubmitted&quot;).val('no');
        document.getElementById('search_form').submit();
    }


            </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;content&quot;)</value>
   </webElementProperties>
</WebElementEntity>
