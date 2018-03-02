//  $(document).ready(function() {
//    $('#vrijwilliger_form').bootstrapValidator({
//        // To use feedback icons, ensure that you use Bootstrap v3.1.0 or later
//        feedbackIcons: {
//            valid: 'glyphicon glyphicon-ok',
//            invalid: 'glyphicon glyphicon-remove',
//            validating: 'glyphicon glyphicon-refresh'
//        },
//        fields: {
//        	voorletters: {
//                validators: {
//                   	stringLength: {
//                   		min: 1,
//                   	},
//                    notEmpty: {
//                    	message: 'Vul je voorletter in'
//                    }
//                }
//        	},
//        	voornaam: {
//                validators: {
//                        stringLength: {
//                        min: 2,
//                    },
//                        notEmpty: {
//                        message: 'Please supply your first name'
//                    }
//                }
//            },
//             achternaam: {
//                validators: {
//                     stringLength: {
//                        min: 2,
//                    },
//                    notEmpty: {
//                        message: 'Please supply your last name'
//                    }
//                }
//            },
//            eMailAdres: {
//                validators: {
//                    notEmpty: {
//                        message: 'Please supply your email address'
//                    },
//                    emailAddress: {
//                        message: 'Please supply a valid email address'
//                    }
//                }
//            }
////            ,
////            phone: {
////                validators: {
////                    notEmpty: {
////                        message: 'Please supply your phone number'
////                    },
////                    phone: {
////                        country: 'US',
////                        message: 'Please supply a vaild phone number with area code'
////                    }
////                }
////            },
////            address: {
////                validators: {
////                     stringLength: {
////                        min: 8,
////                    },
////                    notEmpty: {
////                        message: 'Please supply your street address'
////                    }
////                }
////            },
////            city: {
////                validators: {
////                     stringLength: {
////                        min: 4,
////                    },
////                    notEmpty: {
////                        message: 'Please supply your city'
////                    }
////                }
////            },
////            state: {
////                validators: {
////                    notEmpty: {
////                        message: 'Please select your state'
////                    }
////                }
////            },
////            zip: {
////                validators: {
////                    notEmpty: {
////                        message: 'Please supply your zip code'
////                    },
////                    zipCode: {
////                        country: 'US',
////                        message: 'Please supply a vaild zip code'
////                    }
////                }
////            },
////            comment: {
////                validators: {
////                      stringLength: {
////                        min: 10,
////                        max: 200,
////                        message:'Please enter at least 10 characters and no more than 200'
////                    },
////                    notEmpty: {
////                        message: 'Please supply a description of your project'
////                    }
////                    }
////                }
//            }
//        })
//        .on('success.form.bv', function(e) {
//
// //           $('#success_message').slideDown({ opacity: "show" }, "slow") // Do something ...
//            $('#vrijwilliger_form').data('bootstrapValidator').resetForm();
//
//	        // Prevent form submission
//	        e.preventDefault();
//
//	        // Get the form instance
//	        var $form = $(e.target);
//
//	        // Get the BootstrapValidator instance
//	        var bv = $form.data('bootstrapValidator');
//
////	        var dataArray = { };
////	        $.each($form.serializeArray(), function() {
////	        	if (this.name == "geboortedatum") {
////	        		var t = this.value.split("-");
////	        		dataArray[this.name] = t[2] + "-" + t[1] + "-" + t[0] + "T00:00:00Z";
////	        		console.log(dataArray[this.name]);
////	        	} else {
////	        		dataArray[this.name] = this.value;
////	        	}
////	        	console.log(this.name + " " + dataArray[this.name])
////	        });
////	        $.post("posts", JSON.stringify(dataArray), function(result) {
////	            console.log(result);
////	        }, 'json');
//
//	        var dataArray = { };
//	        $.each($form.serializeArray(), function() {
//        		dataArray[this.name] = this.value;
//	        });
//	        $.post("/user/save", JSON.stringify(dataArray), function(result) {
//	            console.log(result);
//	        }, 'json');
//	        window.location.href = '2017';
//        });
//});
//
//  $(document).ready(function() {
//	    $('#form_year').bootstrapValidator({
//	        // To use feedback icons, ensure that you use Bootstrap v3.1.0 or later
//	        feedbackIcons: {
//	            valid: 'glyphicon glyphicon-ok',
//	            invalid: 'glyphicon glyphicon-remove',
//	            validating: 'glyphicon glyphicon-refresh'
//	        }
//	        })
//	        .on('success.form.bv', function(e) {
//
//	 //           $('#success_message').slideDown({ opacity: "show" }, "slow") // Do something ...
//	            $('#form_year').data('bootstrapValidator').resetForm();
//
//		        // Prevent form submission
//		        e.preventDefault();
//
//		        // Get the form instance
//		        var $form = $(e.target);
//
//		        // Get the BootstrapValidator instance
//		        var bv = $form.data('bootstrapValidator');
//
//
//		        var dataArray = [];
//		        $.each($("table input"), function () {
//		        	var key = this.name.replace("c", "");
//		        	var value = this.checked ? "true" : "false";
//		        	dataArray.push({"calId":key, "aangemeld":value});
//		        });
//		        JSON.stringify(dataArray);
//
//		        $.post("2017", $('form').serializeJSON(), function(result) {
//		            console.log(result);
//		        }, 'json');
//
//	        });
//	});
//
//
  
  $('#radioBtn a').on('click', function(){
	    var sel = $(this).data('title');
	    var tog = $(this).data('toggle');
	    $('#'+tog).prop('value', sel);
	    
	    $('a[data-toggle="'+tog+'"]').not('[data-title="'+sel+'"]').removeClass('btn-primary').addClass('btn-default');
	    $('a[data-toggle="'+tog+'"][data-title="'+sel+'"]').removeClass('btn-default').addClass('btn-primary');
	})
