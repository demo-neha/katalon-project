<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_OrangeHRM</name>
   <tag></tag>
   <elementGuidId>38a5458e-8313-4131-8b44-21efd7659dea</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xmlns</name>
      <type>Main</type>
      <value>http://www.w3.org/1999/xhtml</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xml:lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

        OrangeHRM
        
        
                
        
        
        
        
        
        
        
        
        
        
        
        











   
        
        
                



    

#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}
    
      
        
            
            
                
                
                Welcome Steven
                
                    
                        
    About

    
        
            ×
            About
        
        
            
                
                    
                        
                            Company Name: OrangeHRM (Pvt) Ltd
                        
                        
                            Version: Orangehrm OS 4.1
                        
                        
                            Active Employees: 13
                        
                        
                            Employees Terminated: 0
                        
                    
                
                
                        
    



    jQuery(document).ready(function() {
        
               
        jQuery('#aboutDisplayLink').click(function(event) {
            event.stopImmediatePropagation();
            jQuery('#messageToDisplayAbout').css(
                    'display', 'none');
            jQuery('#displayAbout').modal();
            jQuery('#help-menu.panelContainer').attr('style', 'display:block');
            
            var test = jQuery('.panelContainer');
            jQuery('#help-menu.panelContainer').css(
                    'display', 'block');
             
        });

        jQuery('#heartbeatSubmitBtn').click(function(event) {
            event.stopImmediatePropagation();
            jQuery(this).closest('form').ajaxSubmit(function() {
                jQuery('#messageToDisplayAbout').html('Saved');

                if (jQuery('#register_registration').is(':checked')) {
                    jQuery('#registration-section').css(
                            'display', 'none');
                }
                jQuery('#displayAbout').modal('hide');
                jQuery('#messageToDisplayAbout').css(
                        'display', 'block');
                jQuery('#welcome-menu').css('display','none');
            });
        });

        jQuery('#displayAbout').click(function(event) {
            event.stopImmediatePropagation();
        });
        
        jQuery('#heartbeatCancelBtn').click(function(event) {
            event.stopImmediatePropagation();
             jQuery('#welcome-menu').css('display','none');
                 jQuery('#displayAbout').modal('hide');
        });

    })



                        
                        Logout
                    
                
                                


    svg path,
    svg rect{
        fill: #FF6700;
    }
    .svgcl{
        position: relative;
        left: -35px;
        top: -31px;
    }
    


    var inputDatePattern = 'Y-m-d' ;
    var separatorString = 'to';
    $( document ).ready(function() {

        $(&quot;#loader-1&quot;).hide();
        empId = location.href[location.href.length-1];
        dates = $('#startDates').find(&quot;:selected&quot;).text().split(&quot; &quot;+separatorString+&quot; &quot;);
        startDate_timesheet = dates[0]+&quot; 00:00:00&quot;;
        endDate_timesheet   = dates[1]+&quot; 00:00:00&quot;;

        clientId  =     &quot;&quot;;
        clientSecret  = &quot;&quot;;
        clientUrl     = &quot;&quot;;
        successUrl  = &quot;&quot;;
        var timeSheetStatus = $('#timesheet_status').find('h2').text();
        if(timeSheetStatus == 'Status: Approved'){

            $('.syncToggl').hide();
        } else {
            $('.syncToggl').show();
        }

    });

    
    function startSyc() {


        $(&quot;#loader-1&quot;).show();

    $.ajax({

        type: &quot;POST&quot;,
        url: clientUrl,


        data: {
            'grant_type': 'client_credentials',
            'client_id': clientId,
            'client_secret': clientSecret
        },
        contentType: &quot;application/x-www-form-urlencoded&quot;,


        success: function (msg, status, jqXHR) {

            try {

                msg = $.parseJSON(jqXHR.responseText);

            } catch (err) {
                console.log(err);
                showErrorMsg();
            }

            $.ajax({
                type: &quot;POST&quot;,
                url: successUrl,
                beforeSend: function (xhr) {

                    xhr.setRequestHeader(&quot;Authorization&quot;, &quot;Bearer &quot; + msg.access_token);
                },

                data: {

                    'employee_Id':employeeId,
                    'startTime': startDate_timesheet,
                    'endTime': endDate_timesheet,
                    'timeFormat': inputDatePattern,
                    'timeZone': 'GMT'+formatTimeZone()
                },
                contentType: &quot;application/x-www-form-urlencoded&quot;,

                success: function (msg, status, jqXHR) {

                    $(&quot;#loader-1&quot;).hide();
                    msgCode = msg.statusCode;
                    if (msgCode != null) {
                        if (msgCode == 101) {
                            displayMessages('error',msg.description );
                        } else if (msgCode == 102) {

                            displayMessages('success', msg.description);
                            setTimeout(function () {
                                location.reload();
                            }, 2000);

                        }
                    } else {
                        showErrorMsg();
                    }

                },
                error: function (XMLHttpRequest, textStatus, errorThrown) {
                    $(&quot;#loader-1&quot;).hide();
                    console.log(errorThrown);
                    showErrorMsg();
                }
            });

        },
        error: function (XMLHttpRequest, textStatus, errorThrown) {
            $(&quot;#loader-1&quot;).hide();
            console.log(errorThrown);
            showErrorMsg();
        }


    });

    }

    function showErrorMsg(){
        displayMessages('error','Unable To Sync With Toggl' );
        setTimeout(function () {
            $('#msgDiv').remove();
        }, 3000);

    }




    
    
        
            
  
      
  
  
        
    

    
        
            ×
            Confirm Toggl Sync
        
        
            Any existing timesheet entry will be overwritten if record for same date is matched. Click ok to continue.
        
        
            
            
        
    






                   
            
            

    
    
        
                    
        PIM
            
            
                        
                    
                                        
                        Employee List
                        
                                                    
                           
                    
                                                
                                         
            
            
                    
        Leave
            
            
                        
                    
                                        
                        Apply
                        
                                                    
                           
                    
                                        
                        My Leave
                        
                                                    
                           
                    
                                        
                        Entitlements
                        
                                                
                            
                                
                                                                
                                    Employee Entitlements
                                
                                                                
                                    My Entitlements
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Reports
                        
                                                
                            
                                
                                                                
                                    Leave Entitlements and Usage Report
                                
                                                                
                                    My Leave Entitlements and Usage Report
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Leave List
                        
                                                    
                           
                    
                                        
                        Assign Leave
                        
                                                    
                           
                    
                                                
                                         
            
            
                    
        Time
            
            
                                    
                        
                            
                                         
            
            
                    
        Recruitment
            
            
                        
                    
                                        
                        Candidates
                        
                                                    
                           
                    
                                                
                                         
            
            
                    
        My Info
            
            
                                    
                        
                            
                                         
            
            
                    
        Performance
            
            
                        
                    
                                        
                        Manage Reviews
                        
                                                
                            
                                
                                                                
                                    My Reviews
                                
                                                                
                                    Review List
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        My Trackers
                        
                                                    
                           
                    
                                        
                        Employee Trackers
                        
                                                    
                           
                    
                                                
                                         
            
            
                    
        Dashboard
            
            
                                    
                        
                            
                                         
            
            
                    
        Directory
            
            
                                    
                        
                            
                                         
            
            
                    
     
    
 
            

                  

    
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


             
          
         
        
        
            OrangeHRM 4.1
© 2005 - 2018 OrangeHRM, Inc. All rights reserved.
                 
        
        
 
        

            $(document).ready(function() {                            
                
                /* Enabling tooltips */
                $(&quot;.tiptip&quot;).tipTip();

                /* Toggling header menus */
                $(&quot;#welcome&quot;).click(function () {
                    $(&quot;#welcome-menu&quot;).slideToggle(&quot;fast&quot;);
                    $(this).toggleClass(&quot;activated-welcome&quot;);
                    return false;
                });
                
                $(&quot;#help&quot;).click(function () {
                    $(&quot;#help-menu&quot;).slideToggle(&quot;fast&quot;);
                    $(this).toggleClass(&quot;activated-help&quot;);
                    return false;
                });
                
                $('.panelTrigger').outside('click', function() {
                    $('.panelContainer').stop(true, true).slideUp('fast');
                });                

                /* 
                 * Button hovering effects 
                 * Note: we are not using pure css using :hover because :hover applies to even disabled elements.
                 * The pseudo class :enabled is not supported in IE &lt; 9.
                 */                
                $(document).on({
                    mouseenter: function () {
                        $(this).addClass('hover');                        
                    },
                    mouseleave: function () {
                        $(this).removeClass('hover');                        
                    }

                }, 'input[type=button], input[type=submit], input[type=reset]'); 
  
                /* Fading out main messages */
                $(document).on({
                    click: function() {
                        $(this).parent('div.message').fadeOut(&quot;slow&quot;);
                    }
                }, '.message a.messageCloseButton');                

                /* Toggling search form: Begins */
                //$(&quot;.toggableForm .inner&quot;).hide(); // Disabling this makes search forms to be expanded by default.

                $(&quot;.toggableForm .toggle&quot;).click(function () {
                    $(&quot;.toggableForm .inner&quot;).slideToggle('slow', function() {
                        if($(this).is(':hidden')) {
                            $('.toggableForm .tiptip').tipTip({content:'Expand for Options'});
                        } else {
                            $('.toggableForm .tiptip').tipTip({content:'Hide Options'});
                        }
                    });
                    $(this).toggleClass(&quot;activated&quot;);
                });
                /* Toggling search form: Ends */

                /* Enabling/disabling form fields: Begin */
                
                $('form.clickToEditForm input, form.clickToEditForm select, form.clickToEditForm textarea').attr('disabled', 'disabled');
                $('form.clickToEditForm input.calendar').datepicker('disable');
                $('form.clickToEditForm input[type=button]').removeAttr('disabled');
                
                $('form input.editButton').click(function(){
                    $('form.clickToEditForm input, form.clickToEditForm select, form.clickToEditForm textarea').removeAttr('disabled');
                    $('form.clickToEditForm input.calendar').datepicker('enable');
                });
                
                /* Enabling/disabling form fields: End */
                
            });
            
                

    
    


Hide Options/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
</WebElementEntity>
